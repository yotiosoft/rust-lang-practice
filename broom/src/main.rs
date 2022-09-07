/// 9.構造体 p.189

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent
}

/// Broomができる2つの活動
#[derive(Copy, Clone)]
enum BroomIntent {
    FetchWater,
    DumpWater
}

// 値としてBroomを受け取り所有権を得る
fn chop(b: Broom) -> (Broom, Broom) {
    // broom1の大半をbから作り高さだけを半分にする
    // StringはCopyではないので、broom1はbの名前の所有権を得る
    let mut broom1 = Broom { height: b.height / 2, .. b };  // ..で残り要素をそのまま渡す

    // broom2の大半をbroom1から作る
    // StringはCopyではないので、nameを明示的にクローンする
    let mut broom2 = Broom { name: broom1.name.clone(), .. broom1 };

    // それぞれに別の名前を与える
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

fn main() {
    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater
    };

    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.health, 100);

    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.health, 100);
}
