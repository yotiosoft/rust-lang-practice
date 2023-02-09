/// ver.2 - 15. Iterator

fn view_vec() {
    println!("There's:");
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];

    for element in &v {
        println!("{}", element);
    }
}

use rand::random;
use std::iter::from_fn;
/// 両端が[0,1]のどこかに一様に分布するランダムな線分の長さを、1000個作成する
/// このような分散はrand_distrクレートには含まれていないが、簡単に作成可能
fn use_from_fn() {
    let lengths: Vec<f64> = from_fn(|| Some((random::<f64>() - random::<f64>()).abs())).take(1000).collect();   // take(1000)で乱数1000個
    for element in &lengths {
        println!("{}", element);
    }
}

fn fibonacci() -> impl Iterator<Item=usize> {
    let mut state = (0, 1);

    std::iter::from_fn(move || {
        state = (state.1, state.0 + state.1);
        Some(state.0)
    })
}

fn main() {
    view_vec();
    use_from_fn();
    assert_eq!(fibonacci().take(8).collect::<Vec<_>>(), vec![1, 1, 2, 3, 5, 8, 13, 21]);
}
