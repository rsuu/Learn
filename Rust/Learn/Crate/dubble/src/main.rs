use dubble::*;
fn main() {
    let mut player = DoubleBuffered::construct_with(|| A::new());

    player.write().w_hp(1);
    assert_eq!(player.health, 100);
    player.write().w_hp(1);
    player.update();
    assert_eq!(player.health, 102);

    player.write().s_hp(2);
    player.update();
    assert_eq!(player.health, 100);
}
#[derive(Clone)]
pub struct A {
    health: u32,
}
impl A {
    pub fn new() -> Self {
        A { health: 100 }
    }
    pub fn w_hp(&mut self, n: u32) {
        self.health += n
    }
    pub fn s_hp(&mut self, n: u32) {
        self.health -= n
    }
}
