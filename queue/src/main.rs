//! キューを定義するスタック構造(impl) pp.192-195

/// 文字の先入れ先出しキュー
pub struct Queue {
    older: Vec<char>,       // 古い要素、最も古いものが最後
    younger: Vec<char>      // 新しい要素、最も新しいものが最後
}

// 構造体に対するメソッドは構造体ブロックの外で宣言(impl)
impl Queue {
    /// 文字をキューの末尾にpush
    pub fn push(&mut self, c: char) {       // &mutで共有参照を受け取る -> メンバ変数を変更可能に
        self.younger.push(c);
    }

    /// キューの先端から文字をpopする。
    /// popする文字があれば、Some(c)を返す。空ならNoneを返す。
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            // youngerの要素をolderに移し、約束の順番に入れ替える
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        // ここに来たら、olderには何かが入っているはず
        // VecのpopメソッドはOptionを返すので、そのまま返す
        self.older.pop()
    }

    /// selfを変更しないメソッド -> 共有参照を受け取らない
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    /// selfの所有権を取得するメソッド
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }

    /// selfを受け取らないメソッド ... Static Method
    /// 構造体の値にではなく、構造体そのものに関連付けられた関数
    pub fn new() -> Queue {
        Queue { older: Vec::new(), younger: Vec::new() }
    }
}


fn main() {
    let mut q = Queue::new();

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);
    assert_eq!(q.is_empty(), true);

    /* ------------------------------ */

    let mut q2 = Queue::new();

    q2.push('P');
    q2.push('D');
    assert_eq!(q2.pop(), Some('P'));
    q2.push('X');

    let (older, younger) = q2.split();
    // q2自体はsplit()の実行により未定義状態になった(split()はself自体の所有権を受け取るため)
    // Queueの値はq2から移動
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);
}
