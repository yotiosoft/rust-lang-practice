// 6-9 フィールドと要素
// 実装中

fn quicksort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;             // ソートするものがない
    }

    // スライスを前後2つに分ける
    let pivot_index = slice.len() / 2;

    // 再帰的に前半分のスライスをソートする
    quicksort(&mut slice[.. pivot_index]);

    // 後ろ半分もソートする
    quicksort(&mut slice[pivot_index + 1 ..]);
}

fn main() {
    let mut array = [10, 4, 2, 8, 12, 6];
    quicksort(&mut array);

    for element in array {
        println!("{} ", element);
    }
}
