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

impl Entity {
    pub fn new(x: f32, y: f32, texture: Texture2D) -> Entity {
        Entity {x, y, texture}
    }
    pub fn moveEntity(&mut self) {
        self.x += 0.05;

    }
}