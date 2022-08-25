// 5.3 共有と変更

fn main() {
    let mut x = 10;
    let r1 = &x;
    let r2 = &x;        // ok: 複数の共有参照は可能
    // x += 10;         // Err: xは作用されているので代入不可

    // let m = &mut x;  // Err: 変更不能な共有参照として借用されているので、xを可変参照として借用することはできない

    let mut y = 20;
    let m1 = &mut y;
    // let m2 = &mut y; // Err: 可変参照は1度しか借用できない
    // let z = y;       // Err: yは可変で借用されているので使えない
}
