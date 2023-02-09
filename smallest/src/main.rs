// 5.2.4. 返り値としての参照

// 何らかのデータ構造への参照を引数に取り、その構造の一部への参照を返す
// vは少なくとも1つの要素があるとする
fn smallest(v: &[i32]) -> &i32 {    // 明示的に生存期間を書くと：fn smallest<'a>(v: &'a [i32] -> &'a i32) { ... }
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r };
    }
    s
}

fn main() {
    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
        assert_eq!(*s, 0);
    }
    //assert_eq!(*s, 0);      // ダメ：ドロップされた配列の要素を示している
}
