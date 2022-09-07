// ドクテスト(Doc Test)
// p.177
// cargo doc --no-deps --open
// cargo test

use std::ops::Range;

/// 2つのRangeに重なる部分があればtrueを返す
/// 
///     assert_eq!(ranges::overlap(0..7, 3..10), true);
///     assert_eq!(ranges::overlap(1..5, 101..105), false);
/// 
/// どちらかの範囲が空であれば、重なっていないことにする
/// 
///     assert_eq!(ranges::overlap(0..0, 0..10), false);
/// 

pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r2.end && r1.start < r2.end && r2.start < r1.end
}
