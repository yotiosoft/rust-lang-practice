// ↓ モジュール全体のDocument Comment
//! シダ植物の成長を個々の細胞レベルからシミュレートする

pub struct Fern {
    pub size: f64,
    pub grown_rate: f64
}

impl Fern {
    // ↓ Document Comment
    /// シダ植物の1日での成長をシミュレート
    pub fn grow(&mut self) {
        self.size *= 1.0 + self.grown_rate;
    }
}

// ↓ Document Comment
#[doc = "シダ植物シミュレーションを何日分か行う"]
pub fn run_simuration(fern: &mut Fern, days: usize) {
    for _ in 0 .. days {
        fern.grow();
    }
}
