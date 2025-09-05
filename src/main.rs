// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

mod line;
mod framebuffer;
mod maze;
mod player;
mod caster;
mod textures;
mod enemy; // 👈 Agregado

use textures::TextureManager;
use caster::cast_ray;
use player::{Player, process_events};
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use framebuffer::Framebuffer;
use line::line;
use maze::{Maze,load_maze};
use enemy::Enemy; // 👈 Importación del Enemy

use std::f32::consts::PI;

const TRANSPARENT_COLOR: Color = Color::new(0, 0, 0, 0);

fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(Color::RED);

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.set_pixel(x as i32, y as i32);
        }
    }
}

pub fn render_maze(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &mut Player,
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }

    framebuffer.set_current_color(Color::YELLOW);

    let player_size = 10;
    for dx in 0..player_size {
        for dy in 0..player_size {
            framebuffer.set_pixel(
                player.pos.x as i32 + dx,
                player.pos.y as i32 + dy,
            );
        }
    }

    let num_rays = 5;

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let ray_angle = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, &player, ray_angle, block_size, true);
    }
}

pub fn render_3d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &mut Player,
    texture_cache: &TextureManager,
) {
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;

    framebuffer.set_current_color(Color::WHITESMOKE);
    
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;

        let ray_angle = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let angle_diff = ray_angle - player.a;

        let intersect = cast_ray(framebuffer, &maze, &player, ray_angle, block_size, false);

        let d = intersect.distance;
        let impact = intersect.impact;
        
        let corrected_distance = d * angle_diff.cos() as f32;
        let stake_height = (hh as f32 / corrected_distance) * 70.0;
        let half_stake_height = stake_height / 2.0;
        let stake_top = (hh as f32 - half_stake_height) as usize;
        let stake_bottom = (hh as f32 + half_stake_height) as usize;

        for y in stake_top..stake_bottom {
            let tx = intersect.tx;
            let ty = (y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * 128.0; 
            let color = texture_cache.get_pixel_color(impact, tx as u32, ty as u32);

            framebuffer.set_current_color(color);
            framebuffer.set_pixel(i as i32, y as i32);   
        }
    }
}

fn draw_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    enemy: &Enemy,
    texture_manager: &TextureManager
) {
    let sprite_a = (enemy.pos.y - player.pos.y).atan2(enemy.pos.x - player.pos.x);
    let mut angle_diff = sprite_a - player.a;
    while angle_diff > PI {
        angle_diff -= 2.0 * PI;
    }
    while angle_diff < -PI {
        angle_diff += 2.0 * PI;
    }

    if angle_diff.abs() > player.fov / 2.0 {
        return;
    }

    let sprite_d = ((player.pos.x - enemy.pos.x).powi(2) + (player.pos.y - enemy.pos.y).powi(2)).sqrt();

    if sprite_d < 50.0 || sprite_d > 1000.0 {
        return;
    }

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;

    let sprite_size = (screen_height / sprite_d) * 70.0;
    let screen_x = ((angle_diff / player.fov) + 0.5) * screen_width;

    let start_x = (screen_x - sprite_size / 2.0).max(0.0) as usize;
    let start_y = (screen_height / 2.0 - sprite_size / 2.0).max(0.0) as usize;
    let sprite_size_usize = sprite_size as usize;
    let end_x = (start_x + sprite_size_usize).min(framebuffer.width as usize);
    let end_y = (start_y + sprite_size_usize).min(framebuffer.height as usize);

    for x in start_x..end_x {
        for y in start_y..end_y {
            let tx = ((x - start_x) * 128 / sprite_size_usize) as u32;
            let ty = ((y - start_y) * 128 / sprite_size_usize) as u32;

            let color = texture_manager.get_pixel_color(enemy.texture_key, tx, ty);
            
            if color != TRANSPARENT_COLOR {
                framebuffer.set_current_color(color);
                framebuffer.set_pixel(x as i32, y as i32);
            }
        }
    }
}

fn render_enemies(
    framebuffer: &mut Framebuffer,
    player: &Player,
    texture_cache: &TextureManager,
) {
    let enemies = vec![
        Enemy::new(250.0, 250.0, 'b'),
        Enemy::new(350.0, 300.0, 'f'),
        Enemy::new(350.0, 350.0, 'c'),
    ];
    for enemy in enemies {
        draw_sprite(framebuffer, player, &enemy, texture_cache);
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32,  Color::new(50, 50, 100, 255));

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    let maze = load_maze("./maze.txt").expect("Failed to load maze");
    let mut player = Player{ 
        pos: Vector2::new(150.0, 150.0),
        a: (PI / 2.0) as f32,
        fov: (PI / 2.0) as f32,
    }; 

    let mut mode = "3D";
    let texture_cache = TextureManager::new(&mut window, &raylib_thread);
    let delta_time = 0.016; // Aproximadamente 60 FPS

    while !window.window_should_close() {
        framebuffer.clear();
        process_events(&window, &mut player, delta_time);

        if window.is_key_pressed(KeyboardKey::KEY_M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }
        
        if mode == "2D" {
           render_maze(&mut framebuffer, &maze, block_size, &mut player);
        } else {
            render_3d(&mut framebuffer, &maze, block_size, &mut player, &texture_cache);
            render_enemies(&mut framebuffer, &player, &texture_cache);
        }    

        framebuffer.swap_buffers(&mut window, &raylib_thread);
        thread::sleep(Duration::from_millis(16));
    }
}
