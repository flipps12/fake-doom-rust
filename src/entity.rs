use macroquad::texture::Texture2D;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32, // Ángulo de visión
}

pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub texture: Texture2D,
}
