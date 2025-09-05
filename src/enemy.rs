// enemy.rs

use raylib::prelude::*;

/// Estructura básica de un enemigo
pub struct Enemy {
    pub pos: Vector2,    // posición en el mundo
    pub texture_key: char // clave de textura para render
}

impl Enemy {
    /// Crea un nuevo enemigo
    pub fn new(x: f32, y: f32, texture_key: char) -> Self {
        Enemy { pos: Vector2::new(x, y), texture_key }
    }
}
