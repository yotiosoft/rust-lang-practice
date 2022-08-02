extern crate iron;
#[macro_use] extern crate mime; // このクレートでエクスポートされているマクロを使用

use iron::prelude::*;           // iron::preludeで公開されているすべての名前を利用
use iron::status;

fn main() {
    println!("Serving on http://localhost:3000...");
    Iron::new(get_form).http("localhost:3000").unwrap();    // localhost:3000で待受, 全てのリクエストに対してget_form()を利用
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
            <bututon type="submit">Compute GCD</button>
        </form>
    "#);                                                    // 文字列 -> レスポンスのボディ部分 (r#" .. #"で複数行に渡って記述(JavaScriptの`のようなもの))

    Ok(response)
}
