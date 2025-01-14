use std::process::exit;

use macroquad::prelude::*;

mod entity;
mod render;
mod map;
mod utils;

use entity::{ Player, Entity };
use map::calc_shadows;
use render::{ render, render_entity, RenderObjects, RenderWalls, RenderEntity };
use utils::split_into_pairs;

const DEBUG_MODE: bool = true;
struct RayCastingLine {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

#[macroquad::main("Fake doom")]
async fn main() {
    // Window settings
    // set_fullscreen(true);
    set_cursor_grab(false);
    show_mouse(true);

    // Load textures
    let texture_pared = load_texture("assets/pared.png").await.unwrap();
    texture_pared.set_filter(FilterMode::Nearest);
    let texture_techo = load_texture("assets/techo.png").await.unwrap();
    texture_techo.set_filter(FilterMode::Nearest);
    let texture_piso = load_texture("assets/piso.png").await.unwrap();
    texture_piso.set_filter(FilterMode::Nearest);
    let entity_texture = load_texture("assets/cucas.png").await.unwrap();
    entity_texture.set_filter(FilterMode::Nearest);

    let mut player = Player {
        x: 2.0,
        y: 2.0,
        angle: 0.0,
    };

    const FOV: f32 = std::f32::consts::PI / 3.5; // Campo de visión

    let mut last_mouse_x = screen_width() / 2.0;

    let mut entities: Vec<Entity> = vec![
        // cucas
        Entity { x: 8.8, y: 2.0, angle: 1.0, texture: entity_texture.clone() }
    ];

    let mut menu = true;

    calc_shadows();

    // debug
    if DEBUG_MODE {
        let map = map::get_map();

        for row in &map {
            println!("{:?}", row);
        }
    }

    let mut ray_casting_lines: Vec<RayCastingLine> = Vec::new();
    loop {
        // Reset
        clear_background(BLACK);

        let screen_width = screen_width();
        let screen_height = screen_height();

        let map = map::get_map();

        if menu {
            let text = "Fake doom";
            let text1 = "Press Enter to start";
            let font_size = 40;

            // Medir el texto
            let text_dimensions = measure_text(text, None, font_size, 1.0);
            let text_dimensions1 = measure_text(text1, None, font_size, 1.0);

            // Coordenadas centradas
            let text_x = (screen_width - text_dimensions.width) / 2.0;
            let text_x1 = (screen_width - text_dimensions1.width) / 2.0;
            draw_text(&format!("{}", text), text_x, screen_height / 4.0, font_size as f32, WHITE);
            draw_text(&format!("{}", text1), text_x1, screen_height / 2.0, font_size as f32, WHITE);

            if is_key_down(KeyCode::Enter) {
                show_mouse(false);
                set_cursor_grab(true);
                menu = false;
            }
        } else {
            for entity in &mut entities {
                entity.move_entity(player.x, player.y);
            }
            // let mut shadow_textures = 1.0;
            // if let Some(value) = get_value(player.x.round() as usize - 1, player.y.round() as usize) {
            //     shadow_textures = value as f32 / 10.0;
            // }
            // let color: Color = Color::new(shadow_textures, shadow_textures, shadow_textures, 1.0); // Cambiar sombreado segun la intensidad de la luz donde se encuentra
            let color: Color = Color::new(0.7, 0.7, 0.7, 1.0);

            draw_texture_ex(
                &texture_techo,
                0.0,
                0.0,
                color, // Color base
                DrawTextureParams {
                    dest_size: Some(Vec2::new(screen_width, screen_height / 2.0)), // Escalar textura a la pared
                    ..Default::default()
                }
            );

            draw_texture_ex(
                &texture_piso,
                0.0,
                screen_height / 2.0,
                color, // Color base
                DrawTextureParams {
                    dest_size: Some(Vec2::new(screen_width, screen_height / 2.0)), // Escalar textura a la pared
                    ..Default::default()
                }
            );

            // Lanza rayos en un campo de visión
            let num_rays = (screen_width * 1.0) as usize; // Aumenta el número de rayos
            let max_depth = 18.0;
            let line_width = screen_width / (num_rays as f32);
            let mut texture = texture_pared.clone();

            let mut vector_render: Vec<RenderObjects> = Vec::new();

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
                let step_x;
                let mut side_dist_x = if eye_x < 0.0 {
                    step_x = -1;
                    player.x.fract() * delta_dist_x
                } else {
                    step_x = 1;
                    (1.0 - player.x.fract()) * delta_dist_x
                };

                let step_y;
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
                let mut splited_map: Vec<i32> = Vec::new();

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
                    if test_x < map[0].len() && test_y < map.len() && map[test_y][test_x] > 09 {
                        hit = true;

                        splited_map = split_into_pairs(map[test_y][test_x]);
                        texture = match splited_map[0] {
                            // guardar split_into_pairs en una variable
                            2 => texture_techo.clone(),
                            _ => texture_pared.clone(),
                        };

                        if DEBUG_MODE {
                            let start_x = screen_width / 2.0;
                            let start_y = screen_height / 2.0;
                            let end_x = start_x + ray_angle.cos() * distance * 8.0;
                            let end_y = start_y + ray_angle.sin() * distance * 8.0;

                            ray_casting_lines.push(RayCastingLine {
                                x1: start_x,
                                y1: start_y,
                                x2: end_x,
                                y2: end_y,
                            });
                        }
                    }
                }

                if hit {
                    vector_render.push(
                        RenderObjects::new(
                            distance,
                            Some(RenderWalls {
                                distance,
                                player: &player,
                                ray_angle,
                                eye_x,
                                eye_y,
                                hit_vertical,
                                texture: texture.clone(),
                                opacity: (splited_map[1] as f32) * 0.1,
                                i,
                                line_width,
                                screen_height,
                            }),
                            None
                        )
                    );
                }
            }

            for entity in &entities {
                let distance = calculate_distance(&player, entity);
                // println!("{}", distance);
                // Ignorar entidades fuera del rango visible
                if distance < max_depth {
                    let angle_to_entity = (entity.y - player.y).atan2(entity.x - player.x);
                    let angle_diff =
                        (angle_to_entity - player.angle + std::f32::consts::PI).rem_euclid(
                            2.0 * std::f32::consts::PI
                        ) - std::f32::consts::PI;

                    // Si la entidad está dentro del FOV
                    if angle_diff.abs() < FOV / 1.2 {
                        // 2.0
                        let size = screen_height / distance; // Tamaño relativo al jugador
                        let screen_x =
                            screen_width / 2.0 + (angle_diff / (FOV / 2.0)) * (screen_width / 2.0);
                        vector_render.push(
                            RenderObjects::new(
                                distance,
                                None,
                                Some(
                                    Box::new(RenderEntity {
                                        texture: &entity.texture,
                                        screen_x,
                                        screen_y: screen_height / 2.0, // Ajustar según diseño
                                        size,
                                    })
                                )
                            )
                        );
                    }
                }
            }

            vector_render.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());

            for obj in &vector_render {
                if let Some(entity) = &obj.render_wall {
                    render(entity);
                } else if let Some(entity) = &obj.render_entity {
                    render_entity(entity);
                }
            }

            let speed_multiplier = if is_key_down(KeyCode::LeftShift) { 1.5 } else { 1.0 };

            // Movement
            if is_key_down(KeyCode::W) {
                // Collision detection
                let new_x = player.x + player.angle.cos() * 0.35;
                let new_y = player.y + player.angle.sin() * 0.35;

                if map[player.y as usize][new_x as usize] <= 9 {
                    player.x = player.x + player.angle.cos() * 0.07 * speed_multiplier;
                }

                if map[new_y as usize][player.x as usize] <= 9 {
                    player.y = player.y + player.angle.sin() * 0.07 * speed_multiplier;
                }
            }

            if is_key_down(KeyCode::S) {
                let new_x = player.x - player.angle.cos() * 0.35;
                let new_y = player.y - player.angle.sin() * 0.35;

                if map[player.y as usize][new_x as usize] <= 9 {
                    player.x = player.x - player.angle.cos() * 0.07 * speed_multiplier;
                }

                if map[new_y as usize][player.x as usize] <= 9 {
                    player.y = player.y - player.angle.sin() * 0.07 * speed_multiplier;
                }
            }

            if is_key_down(KeyCode::A) {
                let new_x = player.x + (player.angle - std::f32::consts::FRAC_PI_2).cos() * 0.2;
                let new_y = player.y + (player.angle - std::f32::consts::PI / 2.0).sin() * 0.2;

                if map[player.y as usize][new_x as usize] <= 9 {
                    player.x = player.x + (player.angle - std::f32::consts::FRAC_PI_2).cos() * 0.05;
                }
                if map[new_y as usize][player.x as usize] <= 9 {
                    player.y = player.y + (player.angle - std::f32::consts::FRAC_PI_2).sin() * 0.05;
                }
            }

            if is_key_down(KeyCode::D) {
                let new_x = player.x + (player.angle + std::f32::consts::FRAC_PI_2).cos() * 0.2;
                let new_y = player.y + (player.angle + std::f32::consts::FRAC_PI_2).sin() * 0.2;

                if map[player.y as usize][new_x as usize] <= 9 {
                    player.x = player.x + (player.angle + std::f32::consts::FRAC_PI_2).cos() * 0.05;
                }
                if map[new_y as usize][player.x as usize] <= 9 {
                    player.y = player.y + (player.angle + std::f32::consts::FRAC_PI_2).sin() * 0.05;
                }
            }

            // Manipulating Angle
            if is_key_down(KeyCode::Left) {
                player.angle -= 0.05;
            }
            if is_key_down(KeyCode::Right) {
                player.angle += 0.05;
            }
            // Manipulating Angle with Mouse
            let (mouse_x, _mouse_y) = mouse_position();
            let delta_x = mouse_x - last_mouse_x;
            player.angle += delta_x * 0.0025;
            last_mouse_x = mouse_x;

            // Debug
            if DEBUG_MODE {
                for line in &ray_casting_lines {
                    draw_line(
                        line.x1,
                        line.y1,
                        line.x2,
                        line.y2,
                        1.0,
                        Color::new(1.0, 0.0, 0.0, 1.0)
                    );
                }
                ray_casting_lines.clear();

                draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, BLACK);
                draw_text(&format!("X: {}\n", player.x.round()), 10.0, 50.0, 30.0, BLACK);
                draw_text(&format!("Y: {}", player.y.round()), 10.0, 80.0, 30.0, BLACK);
                draw_text(
                    &format!("X: {} Y: {}", entities[0].x.round(), entities[0].y.round()),
                    10.0,
                    110.0,
                    30.0,
                    BLACK
                );
            }
        }

        if is_key_down(KeyCode::Escape) {
            exit(0);
        }

        next_frame().await;
    }
}

fn calculate_distance(player: &Player, entity: &Entity) -> f32 {
    ((entity.x - player.x).powi(2) + (entity.y - player.y).powi(2)).sqrt()
}
