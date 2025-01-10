use macroquad::prelude::*;

use crate::entity::Player;

pub struct RenderObjects<'a> {
    pub distance: f32,
    pub render_wall: Option<RenderWalls<'a>>,
    pub render_entity: Option<Box<RenderEntity<'a>>>,
}

pub struct RenderWalls<'a> {
    pub player: &'a Player,
    pub distance: f32,
    pub ray_angle: f32,
    pub eye_x: f32,
    pub eye_y: f32,
    pub hit_vertical: bool,
    pub texture: Texture2D,
    pub opacity: f32,
    pub i: usize,
    pub line_width: f32,
    pub screen_height: f32,
}

pub struct RenderEntity<'a> {
    pub texture: &'a Texture2D,
    pub screen_x: f32, // Posición en pantalla
    pub screen_y: f32,
    pub size: f32, // Tamaño del sprite en pantalla
}

impl<'a> RenderObjects<'a> {
    pub fn new(
        distance: f32,
        render_wall: Option<RenderWalls<'a>>,
        render_entity: Option<Box<RenderEntity<'a>>>
    ) -> Self {
        RenderObjects {
            distance,
            render_wall,
            render_entity,
        }
    }
}

pub fn render(obj: &RenderWalls) {
    let corrected_distance = obj.distance * (obj.player.angle - obj.ray_angle).cos();
    let line_height = obj.screen_height / corrected_distance;
    let line_start = (obj.screen_height - line_height) / 2.0;

    // Calcula la textura correcta
    let hit_x = obj.player.x + obj.eye_x * obj.distance;
    let hit_y = obj.player.y + obj.eye_y * obj.distance;
    let texture_offset = if obj.hit_vertical { hit_y.fract() } else { hit_x.fract() };

    let shade = obj.opacity; // 1.0 / (1.0 + obj.distance.powi(2) * 0.01);
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

pub fn render_entity(obj: &RenderEntity) {
    let color: Color = Color::new(1.0, 1.0, 1.0, 1.0); // Color sin sombreado
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