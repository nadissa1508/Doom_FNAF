//player.rs
use raylib::prelude::*;
use std::f32::consts::{PI, TAU};

pub struct Player {
    pub pos: Vector2,
    pub a: f32,   // ángulo en radianes
    pub fov: f32, // campo de visión
}

impl Player {
    pub fn new(x: f32, y: f32, fov: f32) -> Self {
        Player {
            pos: Vector2::new(x, y),
            a: 0.0,
            fov,
        }
    }

    pub fn normalize_angle(&mut self) {
        if self.a < 0.0 {
            self.a += TAU;
        } else if self.a >= TAU {
            self.a -= TAU;
        }
    }
}

pub fn process_events(window: &RaylibHandle, player: &mut Player, delta_time: f32) {
    const MOVE_SPEED: f32 = 150.0; // píxeles por segundo
    const ROTATION_SPEED: f32 = PI / 2.0; // rad/s

    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a -= ROTATION_SPEED * delta_time;
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a += ROTATION_SPEED * delta_time;
    }

    if window.is_key_down(KeyboardKey::KEY_UP) {
        player.pos.x += MOVE_SPEED * delta_time * player.a.cos();
        player.pos.y += MOVE_SPEED * delta_time * player.a.sin();
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) {
        player.pos.x -= MOVE_SPEED * delta_time * player.a.cos();
        player.pos.y -= MOVE_SPEED * delta_time * player.a.sin();
    }

    player.normalize_angle();
}
