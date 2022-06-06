use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::fs::File;

fn main() {
    let decoder = png::Decoder::new(File::open("test.png").unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let u32_buffer: Vec<u32> = buf
        .chunks(3)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .collect();

    let mut window = Window::new(
        "Noise Test - Press ESC to exit",
        reader.info().width as usize,
        reader.info().height as usize,
        WindowOptions {
            borderless: true,
            transparency: true,
            title: false,
            resize: true,
            topmost: false,
            none: false,
            scale_mode: ScaleMode::Center,
            scale: Scale::X1,
        },
    )
    .expect("Unable to open Window");

    window.set_position(0, 0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(
                &u32_buffer,
                reader.info().width as usize,
                reader.info().height as usize,
            )
            .unwrap();
    }
}
