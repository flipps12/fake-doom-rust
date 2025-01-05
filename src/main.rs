use macroquad::prelude::*;

mod entity;

use entity::Player;
use miniquad::window::set_window_size;

#[macroquad::main("Fake doom")]
async fn main() {
    set_window_size(800, 600);

    const FOV: f32 = std::f32::consts::PI / 4.0; // Campo de visión
    let texture_pared = load_texture("assets/pared.png").await.unwrap();
    texture_pared.set_filter(FilterMode::Nearest);

    let texture_techo = load_texture("assets/techo.png").await.unwrap();
    texture_techo.set_filter(FilterMode::Nearest);
    let texture_piso = load_texture("assets/piso.png").await.unwrap();
    texture_piso.set_filter(FilterMode::Nearest);

    const MAP: [[i32; 20]; 20] = [
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1],
        [1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let mut player = Player {
        x: 2.0, // Posición inicial en el mapa
        y: 2.0,
        angle: 0.0, // Dirección inicial
    };

    loop {
        clear_background(BLACK);

        let screen_width = screen_width();
        let screen_height = screen_height();

        draw_texture_ex(
            &texture_techo,
            0.0,
            0.0,
            WHITE, // Color base
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width, screen_height / 2.0)), // Escalar textura a la pared
                ..Default::default()
            }
        );

        draw_texture_ex(
            &texture_piso,
            0.0,
            screen_height / 2.0,
            WHITE, // Color base
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width, screen_height / 2.0)), // Escalar textura a la pared
                ..Default::default()
            }
        );

        // Lanza rayos en un campo de visión
        let num_rays = (screen_width * 3.0) as usize; // Aumenta el número de rayos
        let max_depth = 18.0;
        let line_width = screen_width / (num_rays as f32);
        let mut texture = texture_pared.clone();

        for i in 0..num_rays {
            let ray_angle = player.angle - FOV / 2.0 + FOV * ((i as f32) / (num_rays as f32));

            // Dirección del rayo
            let eye_x = ray_angle.cos();
            let eye_y = ray_angle.sin();

            // Posición inicial del rayo
            let mut ray_x = player.x;
            let mut ray_y = player.y;

            // Cálculo del paso en el eje X y Y
            let delta_dist_x = if eye_x != 0.0 { (1.0 / eye_x).abs() } else { f32::INFINITY };
            let delta_dist_y = if eye_y != 0.0 { (1.0 / eye_y).abs() } else { f32::INFINITY };

            // Determina el paso y la primera intersección
            let mut step_x = 0;
            let mut side_dist_x = if eye_x < 0.0 {
                step_x = -1;
                player.x.fract() * delta_dist_x
            } else {
                step_x = 1;
                (1.0 - player.x.fract()) * delta_dist_x
            };

            let mut step_y = 0;
            let mut side_dist_y = if eye_y < 0.0 {
                step_y = -1;
                player.y.fract() * delta_dist_y
            } else {
                step_y = 1;
                (1.0 - player.y.fract()) * delta_dist_y
            };

            // Bucle DDA
            let mut hit = false;
            let mut hit_vertical = false;
            let mut distance = 0.0;

            while !hit && distance < max_depth {
                // Avanza hacia la siguiente intersección
                if side_dist_x < side_dist_y {
                    ray_x += step_x as f32;
                    distance = side_dist_x;
                    side_dist_x += delta_dist_x;
                    hit_vertical = true;
                } else {
                    ray_y += step_y as f32;
                    distance = side_dist_y;
                    side_dist_y += delta_dist_y;
                    hit_vertical = false;
                }

                // Calcula la posición en el mapa
                let test_x = ray_x as usize;
                let test_y = ray_y as usize;

                // Verifica si hemos alcanzado una pared
                if test_x < MAP[0].len() && test_y < MAP.len() && MAP[test_y][test_x] > 0 {
                    hit = true;

                    texture = match MAP[test_y][test_x] {
                        2 => texture_techo.clone(),
                        _ => texture_pared.clone(),
                    };
                }
            }

            if hit {
                // Corrige la distancia para evitar distorsión del "fish-eye"
                let corrected_distance = distance * (player.angle - ray_angle).cos();
                let line_height = screen_height / corrected_distance;
                let line_start = (screen_height - line_height) / 2.0;

                // Calcula la textura correcta
                let hit_x = player.x + eye_x * distance;
                let hit_y = player.y + eye_y * distance;
                let texture_offset = if hit_vertical { hit_y.fract() } else { hit_x.fract() };

                let shade = 1.0 / (1.0 + distance.powi(2) * 0.01);
                let color = Color::new(shade, shade, shade, 1.0);

                draw_texture_ex(
                    &texture,
                    (i as f32) * line_width,
                    line_start,
                    color,
                    DrawTextureParams {
                        source: Some(
                            Rect::new(
                                texture_offset * (texture.width() as f32),
                                0.0,
                                line_width,
                                texture.height() as f32
                            )
                        ),
                        dest_size: Some(Vec2::new(line_width, line_height)),
                        ..Default::default()
                    }
                );
            }
        }

        // Movimiento del jugador (WASD)
        if is_key_down(KeyCode::W) {
            let new_x = player.x + player.angle.cos() * 0.1;
            let new_y = player.y + player.angle.sin() * 0.1;

            // Verificar colisiones
            if MAP[new_y as usize][new_x as usize] == 0 {
                player.x = new_x;
                player.y = new_y;
            }
        }
        if is_key_down(KeyCode::S) {
            let new_x = player.x - player.angle.cos() * 0.1;
            let new_y = player.y - player.angle.sin() * 0.1;

            if MAP[new_y as usize][new_x as usize] == 0 {
                player.x = new_x;
                player.y = new_y;
            }
        }
        if is_key_down(KeyCode::A) {
            player.angle -= 0.05;
        }
        if is_key_down(KeyCode::D) {
            player.angle += 0.05;
        }

        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, BLACK);
        draw_text(&format!("X: {}\n", player.x.round()), 10.0, 50.0, 30.0, BLACK);
        draw_text(&format!("Y: {}", player.y.round()), 10.0, 80.0, 30.0, BLACK);
        next_frame().await;
    }
}
