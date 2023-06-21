/*
このコードはコンパイルできない：
struct S {
    r: &i32
}
*/

// 参照型が型定義に含まれている場合、その生存期間を明示的に書かなければならない
struct S<'a> {
    r: &'a i32      // &'a: 参照状況に応じて決定 or r: &'static i32 : いくらでも生存できる
}

fn main() {
    let s;
    {
        let x = 10;
        s = S { r: &x };    // 'aはsの生存期間より長生きしなければならないという制約が課される
        assert_eq!(*s.r, 10);
    }
}