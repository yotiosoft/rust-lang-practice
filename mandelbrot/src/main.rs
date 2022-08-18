extern crate num;
use num::Complex;
use std::str::FromStr;

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
fn pixel_to_print(bounds: (usize, usize),
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
fn test_pixel_to_print() {
    assert_eq!(pixel_to_print((100, 100), (25, 75),
                                Complex { re: -1.0, im:  1.0 },
                                Complex { re:  1.0, im: -1.0 }),
                Complex { re: -0.5, im: -0.5 };)
}

fn main() {
    
}
