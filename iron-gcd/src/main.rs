extern crate iron;
extern crate router;
extern crate urlencoded;
#[macro_use] extern crate mime; // このクレートでエクスポートされているマクロを使用

use iron::prelude::*;           // iron::preludeで公開されているすべての名前を利用
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;
use std::str::FromStr;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();    // localhost:3000で待受, 全てのリクエストに対してget_form()を利用
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

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing from data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };

    let unparsed_numbers = match form_data.get("\n") {
        Node => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(
                    format!("Value for 'n' parameter not a number: {:?}\n", 
                    unparsed));
                return Ok(response);
            }
            Ok(n) => { numbers.push(n); }
        }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        format!("The gratest common divisor of the numbers {:?} is <b>{}</b>\n",
                            numbers, d)
    );
    Ok(response)
}
