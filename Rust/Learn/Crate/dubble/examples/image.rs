use image::GenericImageView;
use ufb::{ColorDepth, Window};

use dubble::*;
fn main() {
    let img = image::open("screenshots/image.jpg").unwrap();
    let (w, h) = img.dimensions();
    let mut win = Window::new(w, h, ColorDepth::Rgba8, "image.jpg").unwrap();

    win.get_frame().copy_from_slice(&img.to_rgba8());
    win.show();
}
fn a() {
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
