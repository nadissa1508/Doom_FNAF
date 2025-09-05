// framebuffer.rs
use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    pub z_buffer: Vec<f32>,
    background_color: Color,
    current_color: Color,
    texture: Option<Texture2D>, // Textura persistente para la GPU
}

impl Framebuffer {
    /// Crea un nuevo framebuffer con un Z-buffer inicializado
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);

        let z_buffer = vec![f32::INFINITY; (width * height) as usize];

        Self {
            width,
            height,
            color_buffer,
            z_buffer,
            background_color,
            current_color: Color::WHITE,
            texture: None,
        }
    }

    /// Limpia el framebuffer y el Z-buffer sin crear una nueva imagen
    pub fn clear(&mut self) {
        self.color_buffer.clear_background(self.background_color);
        self.z_buffer.fill(f32::INFINITY);
    }

    /// Dibuja un píxel con verificación de límites
    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.color_buffer.draw_pixel(x, y, self.current_color);
        }
    }

    /// Dibuja un píxel con Z-buffer (solo si es más cercano)
    pub fn set_pixel_depth(&mut self, x: i32, y: i32, depth: f32) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            let idx = (y as u32 * self.width + x as u32) as usize;
            if depth < self.z_buffer[idx] {
                self.z_buffer[idx] = depth;
                self.color_buffer.draw_pixel(x, y, self.current_color);
            }
        }
    }

    /// Cambia el color de fondo
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    /// Cambia el color actual de dibujo
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    /// Guarda el framebuffer en un archivo
    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }

    /// Inicializa la textura persistente (solo la primera vez)
    pub fn init_texture(&mut self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        if self.texture.is_none() {
            if let Ok(tex) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
                self.texture = Some(tex);
            }
        }
    }

    /// Actualiza la textura en la GPU y la dibuja en pantalla
    pub fn swap_buffers(&mut self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        // Inicializar textura si no existe
        if self.texture.is_none() {
            self.init_texture(window, raylib_thread);
        }

        if let Some(tex) = &mut self.texture {
            // Obtenemos los píxeles como Vec<Color>
            let pixels: Vec<Color> = self.color_buffer.get_image_data().to_vec();;

            // Convertimos Vec<Color> -> Vec<u8> (RGBA)
            let mut raw: Vec<u8> = Vec::with_capacity(pixels.len() * 4);
            for c in pixels {
                raw.push(c.r);
                raw.push(c.g);
                raw.push(c.b);
                raw.push(c.a);
            }

            tex.update_texture_rec(
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: tex.width() as f32,
                    height: tex.height() as f32,
                },
                &raw,
            );

            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.clear_background(Color::BLACK);
            renderer.draw_texture(tex, 0, 0, Color::WHITE);
        }
    }
}
