static mut STASH: &i32 = &10;       // 可変なstaticは安全ではない：unsafeブロック内でしかアクセスできない

fn f(p: &'static i32) {             // 'static: 生存期間パラメータ（任意の生存期間）を持つことを示す
    unsafe {
        STASH = p;
    }
}

fn main() {
    f(&100);
}
