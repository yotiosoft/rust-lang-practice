fn main() {
    // 文字列"101", ..., "105"のベクタを作る
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }

    // 1. ベクタの最後から値をポップして取り出す
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");

    // 2. ベクタの真ん中の値を引き出し、最後の要素がそこに代わりに入るようにする
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // 3. 取り出した値の代わりに別の値を入れる
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    // ベクタに残ったものを見てみる
    for e in v {
        println!("{} ", e);
    }
}
