use macroquad::prelude::*;

mod entity;

use entity::{ Player, Entity };
use miniquad::window::set_window_size;

struct RenderObjects<'a> {
    distance: f32,
    wall: bool,
    render_wall: Option<RenderWalls<'a>>,
    render_entity: Option<Box<RenderEntity<'a>>>,
}

struct RenderWalls<'a> {
    distance: f32,
    player: &'a Player,
    ray_angle: f32,
    eye_x: f32,
    eye_y: f32,
    hit_vertical: bool,
    texture: Texture2D,
    i: usize,
    line_width: f32,
    screen_height: f32,
}

struct RenderEntity<'a> {
    distance: f32,
    texture: &'a Texture2D,
    screen_x: f32, // Posición en pantalla
    screen_y: f32,
    size: f32, // Tamaño del sprite en pantalla
}

impl<'a> RenderObjects<'a> {
    fn new(
        distance: f32,
        wall: bool,
        render_wall: Option<RenderWalls<'a>>,
        render_entity: Option<Box<RenderEntity<'a>>>
    ) -> Self {
        RenderObjects {
            distance,
            wall,
            render_wall,
            render_entity,
        }
    }
}

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

    let entity_texture = load_texture("assets/cucas.png").await.unwrap();
    entity_texture.set_filter(FilterMode::Nearest);

    let mut entities = vec![
        // cucas
        Entity { x: 8.0, y: 2.0, texture: entity_texture.clone() },
    ];

    loop {
        clear_background(BLACK);

        let screen_width = screen_width();
        let screen_height = screen_height();

        // entities[0].x += 0.03;

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
                vector_render.push(
                    RenderObjects::new(
                        distance,
                        true,
                        Some(RenderWalls {
                            distance,
                            player: &player,
                            ray_angle,
                            eye_x,
                            eye_y,
                            hit_vertical,
                            texture: texture.clone(),
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
            println!("{}", distance);
            // Ignorar entidades fuera del rango visible
            if distance < max_depth {
                let angle_to_entity = (entity.y - player.y).atan2(entity.x - player.x);
                let angle_diff = (angle_to_entity - player.angle + std::f32::consts::PI) % (2.0 * std::f32::consts::PI) - std::f32::consts::PI;

                // Si la entidad está dentro del FOV
                if angle_diff.abs() < FOV / 2.0 {
                    let size = screen_height / distance; // Tamaño relativo al jugador
                    let screen_x =
                        screen_width / 2.0 + (angle_diff / (FOV / 2.0)) * (screen_width / 2.0);
                    vector_render.push(
                        RenderObjects::new(
                            distance,
                            false,
                            None,
                            Some(
                                Box::new(RenderEntity {
                                    distance,
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
            if obj.wall {
                if let Some(entity) = &obj.render_wall {
                    render(entity);
                } else {
                    // Si no tiene entidad, puedes decidir qué hacer
                    println!("No entity to render");
                }
            } else {
                // Verificar si render_entity es Some o None
                if let Some(entity) = &obj.render_entity {
                    render_entity(entity);
                } else {
                    // Si no tiene entidad, puedes decidir qué hacer
                    println!("No entity to render");
                }
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

fn render(obj: &RenderWalls) {
    let corrected_distance = obj.distance * (obj.player.angle - obj.ray_angle).cos();
    let line_height = obj.screen_height / corrected_distance;
    let line_start = (obj.screen_height - line_height) / 2.0;

    // Calcula la textura correcta
    let hit_x = obj.player.x + obj.eye_x * obj.distance;
    let hit_y = obj.player.y + obj.eye_y * obj.distance;
    let texture_offset = if obj.hit_vertical { hit_y.fract() } else { hit_x.fract() };

    let shade = 1.0 / (1.0 + obj.distance.powi(2) * 0.01);
    let color = Color::new(shade, shade, shade, 1.0);

    draw_texture_ex(
        &obj.texture,
        (obj.i as f32) * obj.line_width,
        line_start,
        color,
        DrawTextureParams {
            source: Some(
                Rect::new(
                    texture_offset * (obj.texture.width() as f32),
                    0.0,
                    obj.line_width,
                    obj.texture.height() as f32
                )
            ),
            dest_size: Some(Vec2::new(obj.line_width, line_height)),
            ..Default::default()
        }
    );
}

fn calculate_distance(player: &Player, entity: &Entity) -> f32 {
    ((entity.x - player.x).powi(2) + (entity.y - player.y).powi(2)).sqrt()
}

fn render_entity(obj: &RenderEntity) {
    let color = Color::new(1.0, 1.0, 1.0, 1.0); // Color sin sombreado
    draw_texture_ex(
        obj.texture,
        obj.screen_x - obj.size / 2.0, // Centra el sprite
        obj.screen_y - obj.size / 2.0,
        color,
        DrawTextureParams {
            dest_size: Some(Vec2::new(obj.size, obj.size)),
            ..Default::default()
        }
    );
}
