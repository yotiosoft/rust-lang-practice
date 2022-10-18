extern crate iron;
extern crate router;
extern crate urlencoded;
#[macro_use] extern crate mime; // このクレートでエクスポートされているマクロを使用

use iron::prelude::*;           // iron::preludeで公開されているすべての名前を利用
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;
use std::str::FromStr;

struct Request {
    method: String,
    url: String,
    header: HashMap<String, String>,
    body: Vec<u8>
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

struct BasicRouter<C> where C: Fn(&Request) -> Response {
    routes: HashMap<String, C>
}

impl<C> BasicRouter<C> where C: Fn(&Request) -> Response {
    // 空のルータを作成
    fn new() -> BasicRouter<C> {
        BasicRouter { routes: HashMap::new() }
    }

    // ルータにルートを追加
    fn add_route(&mut self, url: &str, callback: C) {
        self.routes.insert(url.to_string(), callback);
    }
}

fn main() {
    let mut router = BasicRouter::new();
    router.add_route("/", |_| get_from_response());
}

fn get_form(_request: &mut Request) -> IronResult<Response> {   //_requestは後で使う(_で使わないことをコンパイラに示す)
    let mut response = Response::new();

    response.set_mut(status::Ok);                           // srarus::Ok -> HTTPステータス
    response.set_mut(mime!(Text/Html; Charset=Utf8));       // メディアタイプ -> Content-Typeヘッダ
    response.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Compute GCD</button>
        </form>
    "#);                                                    // 文字列 -> レスポンスのボディ部分 (r#" .. #"で複数行に渡って記述(JavaScriptの`のようなもの))

    Ok(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    // パターンマッチング
    let form_data = match request.get_ref::<UrlEncodedBody>() {      // リクエストのボディをパース
        Err(e) => {                                                               // エラー処理（エラーメッセージを返す）
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing from data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map                                               // 成功時、クエリのパラメータ名と値の配列をテーブルに変換
    };

    let unparsed_numbers = match form_data.get("n") {                                 // クエリパラメータ'n'があるか？
        None => {                                                                                   // なければエラーメッセージを返す
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums                                                            // あればその文字列ベクタを返す
    };

    let mut numbers = Vec::new();                                                         // 文字列ベクタを1つずつパース -> 符号なし64ビット変数に
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {                                                                             // 1つでも失敗したらエラー
                response.set_mut(status::BadRequest);
                response.set_mut(
                    format!("Value for 'n' parameter not a number: {:?}\n", 
                    unparsed));
                return Ok(response);
            }
            Ok(n) => { numbers.push(n); }                                                      // 成功したらu64ベクタへ
        }
    }

    // gcdを求める
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    // レスポンスを生成
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        format!("The gratest common divisor of the numbers {:?} is <b>{}</b>\n",
                            numbers, d)         // format!: ストリームにテキストを書き出さずに文字列自体を返すマクロ
    );
    Ok(response)
}

