// caster.rs
use raylib::prelude::Color; // más estable que raylib::color::Color
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::Maze;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub tx: usize,
    // pub ty: usize, // por si luego texturas 2D
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    a: f32,
    block_size: usize,
    draw: bool,
) -> Intersect {
    // --- Configuración y normalización ---
    let map_w = maze.first().map(|r| r.len()).unwrap_or(0);
    let map_h = maze.len();
    let block = block_size as f32;

    // Posición inicial del rayo en coordenadas de mundo (float, no castear aún)
    let mut rx = player.pos.x;
    let mut ry = player.pos.y;

    // Dirección del rayo
    let rdx = a.cos();
    let rdy = a.sin();

    // Evita división por 0
    let inv_rdx = if rdx.abs() < 1e-6 { 1e6 } else { 1.0 / rdx };
    let inv_rdy = if rdy.abs() < 1e-6 { 1e6 } else { 1.0 / rdy };

    // Convertir posición a índices de celda
    let mut map_x = (rx / block).floor() as isize;
    let mut map_y = (ry / block).floor() as isize;

    // Paso en x/y según el signo de la dirección
    let step_x: isize = if rdx < 0.0 { -1 } else { 1 };
    let step_y: isize = if rdy < 0.0 { -1 } else { 1 };

    // Distancia desde rx/ry al primer gridline vertical/horizontal
    let mut side_dist_x: f32;
    let mut side_dist_y: f32;

    // Distancia entre gridlines en ray units
    let delta_dist_x = (1.0_f32).abs() * (block * inv_rdx).abs();
    let delta_dist_y = (1.0_f32).abs() * (block * inv_rdy).abs();

    // Calcular side_dist_* hasta la primera pared virtual
    if rdx < 0.0 {
        side_dist_x = ((rx - map_x as f32 * block)) * inv_rdx.abs();
    } else {
        side_dist_x = (((map_x + 1) as f32 * block) - rx) * inv_rdx.abs();
    }
    if rdy < 0.0 {
        side_dist_y = ((ry - map_y as f32 * block)) * inv_rdy.abs();
    } else {
        side_dist_y = (((map_y + 1) as f32 * block) - ry) * inv_rdy.abs();
    }

    // Para dibujar la línea del rayo si draw==true
    framebuffer.set_current_color(Color::WHITE);

    // Resultados
    let mut hit = false;
    let mut hit_side_x = false; // true: pared vertical (se avanzó en X), false: pared horizontal
    let mut tile: char = ' ';

    // Límite de seguridad
    let max_steps = (map_w + map_h) * 4;
    let mut steps = 0;

    // DDA loop
    while steps < max_steps {
        steps += 1;

        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            hit_side_x = true;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            hit_side_x = false;
        }

        // Bounds check
        if map_x < 0 || map_y < 0 || map_x as usize >= map_w || map_y as usize >= map_h {
            break; // fuera del mapa
        }

        tile = maze[map_y as usize][map_x as usize];
        if tile != ' ' {
            hit = true;
            break;
        }

        // Dibujo opcional del rayo (marcando el centro de la celda)
        if draw {
            let cx = (map_x as f32 * block + block * 0.5) as i32;
            let cy = (map_y as f32 * block + block * 0.5) as i32;
            if cx >= 0 && cy >= 0 {
                framebuffer.set_pixel(cx as i32, cy as i32);
            }
        }
    }

    if !hit {
        // No chocó con nada: devuelve un “miss” largo y vacío
        return Intersect {
            distance: 10_000.0,
            impact: ' ',
            tx: 0,
        };
    }

    // Distancia perpendicular a la pared para evitar fisheye
    let perp_dist = if hit_side_x {
        // Nos movimos en X: pared vertical
        let wall_x = map_x as f32 * block;
        ((wall_x - rx) + if step_x < 0 { block } else { 0.0 }) / rdx
    } else {
        // Nos movimos en Y: pared horizontal
        let wall_y = map_y as f32 * block;
        ((wall_y - ry) + if step_y < 0 { block } else { 0.0 }) / rdy
    }.abs();

    // Cálculo de coordenada de textura (tx en 0..127)
    // Hallamos el punto exacto de impacto en coordenadas de mundo y tomamos su fracción dentro del bloque.
    let hit_world_x = rx + rdx * perp_dist;
    let hit_world_y = ry + rdy * perp_dist;

    let mut tex_u = if hit_side_x {
        // choca pared vertical → variación a lo largo de Y
        (hit_world_y / block) - (hit_world_y / block).floor()
    } else {
        // choca pared horizontal → variación a lo largo de X
        (hit_world_x / block) - (hit_world_x / block).floor()
    };

    // Opcional: invertir para caras “mirando” hacia el otro lado
    if (hit_side_x && rdx > 0.0) || (!hit_side_x && rdy < 0.0) {
        tex_u = 1.0 - tex_u;
    }

    let tx = (tex_u * 128.0) as usize % 128;

    Intersect {
        distance: perp_dist.max(0.0001),
        impact: tile,
        tx,
    }
}
