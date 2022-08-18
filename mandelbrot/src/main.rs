extern crate num;
extern crate image;
extern crate crossbeam;     // 並列機構

use num::Complex;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::io::Write;

/// <- ドキュメントコメント
/// 'limit'を繰り返し回数の上限として、'c'がマンデルブロ集合に含まれるかを判断する。
/// 'c'がマンデルブロ集合に含まれないなら'Some(i)'を返す。'i'は'c'が原点を中心とする半径2の円から出るまでにかかった繰り返し回数となる。
/// 'c'がマンデルブロ集合に含まれているらしい場合（正確に言うと、繰り返し回数の上限に達しても'c'がマンデルブロ集合に含まれないことを示せなかった場合）、'None'を返す。
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {             // limit-1まで繰り返し
        z = z * z + c;
        if z.norm_sqr() > 4.0 {     // この計算方法のほうが高速（平方根を計算しない）
            return Some(i);         // OptionのSome(T) -> 有効値（型Tの値が得られた）, マンデルブロ集合に含まれていなさそう
        }
    }

    None                            // OptionのNone -> 無効値（型Tの値が得られなかった）, マンデルブロ集合に含まれていそう
}

/// 文字列sは座標系のペア。"400x600", "1.0,0.5" など。
/// sは<left><sep><right>の形でなければならない。<sep>は'separator'引数で与えられる文字で、<left>と<right>は双方とも'T::from_str'でパースできる文字列
/// sが適切な形であればSome<(x, y)>を返す。パースできなければNoneを返す。
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",        ","), None);
    assert_eq!(parse_pair::<i32>("10,",     ","), None);
    assert_eq!(parse_pair::<i32>(",10,",    ","), None);
    assert_eq!(parse_pair::<i32>("10,20",   ","), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ","), None);
    assert_eq!(parse_pair::<f64>("0.5x",    ","), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5,","x"), Some(0.5, 1.5));
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, '.') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"),
                Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625", None));
}

// 出力される画像のピクセルの位置を取り、対応する複素平面上の点を返す
// boundsは出力画像の幅と高さをピクセル単位で与える。
// pixelは画像上の特定のピクセルを(行, 列)ペアの形で指定する。
// 仮引数upper_left, lower_rightは、出力画像に描画する複素平面を左上と右下で指定する。
fn pixel_to_point(bounds: (usize, usize),
                pixel: (usize, usize),
                upper_left: Complex<f64>,
                lower_right: Complex<f64>)
    -> Complex<f64>
{
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + pixel.0 as f64 * width  / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64  // ここが引き算となっているのはなぜか？上に動くとpixel.1は増えるが、虚部は小さくなるからだ。
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 100), (25, 75),
                                Complex { re: -1.0, im:  1.0 },
                                Complex { re:  1.0, im: -1.0 }),
                Complex { re: -0.5, im: -0.5 };)
}

// 矩形範囲のマンデルブロ集合をピクセルのバッファに描画する
// 仮引数boundsはバッファpixelsの幅と高さを指定する
// バッファpixelsはピクセルのグレースケールの幅をバイトで保持する
// upper_leftとlower_rightはピクセルバッファの左上と右下に対応する複素平面上の点を指定する　
fn render(pixels: &mut [u8],
        bounds: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0 .. bounds.1 {
        for column in 0 .. bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = 
                match escape_time(point, 255) {         
                    None => 0,                          // pointがマンデルブロ集合に含まれると判断したら黒に
                    Some(count) => 255 - count as u8    // それ以外なら円から抜けるのに長くかかった点により暗い色を割り当て
                };
        }
    }
}

// 大きさがboundsで指定したバッファpixelsをfilenameで指定されたバッファに書き出す
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {   // 失敗しうる関数はResult値を返さなければならない
    let output = File::create(filename)?;                   // ?: 呼び出し先で失敗したら即座にそのエラーコードを返す（Err(e)の記述を省略）

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, 
                    bounds.0 as u32, bounds.1 as u32,
                    ColorType::Gray(8))?;

    Ok(())          // 成功時: ()で何も返さないことを示す（ユニット; Cでいうvoid）
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 5 {
        writeln!(std::io::stderr(),
                "Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRiGHT")
            .unwrap();
        writeln!(std::io::stderr(),
                "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
                args[0])
            .unwrap();
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x')
        .expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3])
        .expect("error parsing upper left corner point");
    let lower_right = parse_complex(&arg[4])
        .expect("error parsing lower right corner point");
    
    let mut pixels = vec![0; bounds.0 * bounds.1];      // 長さbounds.0 * bounds.1, 0で初期化

    //render(&mut pixels, bounds, upper_left, lower_right);   // &mut pixels: pixelsの可変参照を借用
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;     // 帯状の領域に何行のピクセルが入るか？

    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();  // pixelsを帯に分割（chunks_mut: 可変な重なり合わないベクタのスライスを生成）
        crossbeam::scope(|spawner| {                            // クロージャ式：関数のように呼び出せる値、|spawner|は引数リスト（型宣言の必要なし）で{...}は関数のボディ; crossbeam::scopeでクロージャ呼び出し、全スレッドの終了を待つ
            for (i, band) in bands.into_iter().enumerate() {    // pixel bufferの帯領域に対してループ; into_iter()で得られるイテレータは帯領域の排他的な所有権をループ内のボディに与える -> 各スレッドに排他的に与えられる; enumerateアダプタは各要素に対してindexと組み合わせたタプルを作る
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left  = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move || {         // スレッドを生成しクロージャmove || {...}を実行、moveによりクロージャの持つbandの所有権を取得する
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        });
    }

    write_image(&args[1], &pixels, bounds)              // &pixels: pixelsの不変な共有参照（変更の必要がないため）
        .expect("error write PNG file");
}
