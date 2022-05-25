use image::GenericImageView;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::cmp;

pub trait DrawExt {
    fn draw_to_buffer(&self, dst: &mut [u32], dst_width: usize, offset: (i32, i32));
}

impl DrawExt for image::RgbImage {
    fn draw_to_buffer(&self, dst: &mut [u32], dst_width: usize, offset: (i32, i32)) {
        let dst_size = (dst_width as i32, (dst.len() / dst_width) as i32);

        let (width, height) = self.dimensions();

        // Make sure only the pixels get rendered that are inside the dst
        let min_x = cmp::max(-offset.0, 0);
        let min_y = cmp::max(-offset.1, 0);

        let max_x = cmp::min(dst_size.0 - offset.0, width as i32);
        let max_y = cmp::min(dst_size.1 - offset.1, height as i32);

        for y in min_y..max_y {
            for x in min_x..max_x {
                let pixel = self.get_pixel(x as u32, y as u32);

                // Convert pixel to Color
                let raw = 0xFF000000
                    | ((pixel[0] as u32) << 16)
                    | ((pixel[1] as u32) << 8)
                    | (pixel[2] as u32);

                // Apply the offsets
                let dst_x = (x + offset.0) as usize;
                let dst_y = (y + offset.1) as usize;

                // Calculate the index
                let index = dst_x + dst_y * dst_size.0 as usize;
                dst[index] = raw;
            }
        }
    }
}

pub struct Winmeta {
    show_width: usize, // display size of window
    show_height: usize,

    pos_x: isize, // position of window
    pos_y: isize,

    img_path: image::DynamicImage, // path of image
}

pub struct Imgmeta {
    size: u64, // size of the image(bytes)

    w_h: (u32, u32), // width and height of image
}
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let winmeta = Winmeta {
        show_width: args[1].parse::<usize>().unwrap(),
        show_height: args[2].parse::<usize>().unwrap(),

        pos_x: args[3].parse::<isize>().unwrap(),
        pos_y: args[4].parse::<isize>().unwrap(),

        img_path: image::open(&args[5]).unwrap(),
    };

    let imgmeta = Imgmeta {
        size: std::fs::metadata(&args[5]).unwrap().len(),

        w_h: winmeta.img_path.dimensions(),
    };

    let windowoptions = minifb::WindowOptions {
        borderless: true,
        transparency: true,
        title: false,
        resize: false,
        topmost: false,
        none: false,
        scale_mode: ScaleMode::Center,
        scale: Scale::FitScreen,
    };

    let mut buffer: Vec<u32> = vec![0x00FFFFFF; winmeta.show_width * winmeta.show_height];

    let rgb = winmeta.img_path.as_rgb8().unwrap();

    rgb.draw_to_buffer(&mut buffer, winmeta.show_width, (0, 0));

    let mut window = Window::new(
        "test",
        winmeta.show_width,
        winmeta.show_height,
        windowoptions,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // sets the position of the window.
        window.set_position(winmeta.pos_x, winmeta.pos_y);

        // create a new window.
        window
            .update_with_buffer(&buffer, winmeta.show_width, winmeta.show_height)
            .unwrap();
    }
}
