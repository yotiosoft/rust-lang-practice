/// ver.2 - 15. Iterator

fn normal_triangle(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

fn ite_triangle(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

fn main() {
    let normal = normal_triangle(100);
    let iter = ite_triangle(100);
    
    println!("Normal: {}", normal);
    println!("Iter: {}", iter);
}
