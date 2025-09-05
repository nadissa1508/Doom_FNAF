// textures.rs

use raylib::prelude::*;
use std::collections::HashMap;
use std::slice;

pub struct TextureManager {
    images: HashMap<char, Image>,       // para muestreo manual
    textures: HashMap<char, Texture2D>, // para dibujar en pantalla
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let texture_files = vec![
            ('.', "assets/wall_corner.png"),
            ('+', "assets/wall_fredy.png"),
            ('-', "assets/wall_chica.png"),
            ('|', "assets/wall_foxy.png"),
            ('g', "assets/wall_draws.png"),
            ('b', "assets/bonnie.png"),
            ('c', "assets/chica.png"),
            ('f', "assets/freddy.png"),
        ];

        let mut images = HashMap::new();
        let mut textures = HashMap::new();

        for (ch, path) in texture_files {
            let image = Image::load_image(path)
                .unwrap_or_else(|_| panic!("Failed to load image {}", path));
            let texture = rl.load_texture(thread, path)
                .unwrap_or_else(|_| panic!("Failed to load texture {}", path));

            images.insert(ch, image);
            textures.insert(ch, texture);
        }

        TextureManager { images, textures }
    }

    pub fn get_pixel_color(&self, ch: char, tx: u32, ty: u32) -> Color {
        if let Some(image) = self.images.get(&ch) {
            let x = tx.min(image.width as u32 - 1) as i32;
            let y = ty.min(image.height as u32 - 1) as i32;
            sample_pixel(image, x, y)
        } else {
            Color::WHITE
        }
    }

    pub fn get_texture(&self, ch: char) -> Option<&Texture2D> {
        self.textures.get(&ch)
    }
}

fn sample_pixel(image: &Image, x: i32, y: i32) -> Color {
    let width = image.width as usize;
    let height = image.height as usize;

    if x < 0 || y < 0 || x as usize >= width || y as usize >= height {
        return Color::WHITE;
    }

    let data_len = width * height * 4;

    unsafe {
        let data = slice::from_raw_parts(image.data as *const u8, data_len);
        let idx = (y as usize * width + x as usize) * 4;

        if idx + 3 >= data_len {
            return Color::WHITE;
        }

        Color::new(data[idx], data[idx + 1], data[idx + 2], data[idx + 3])
    }
}
