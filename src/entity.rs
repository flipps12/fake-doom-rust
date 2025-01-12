use macroquad::texture::Texture2D;

use crate::map;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32, // Ángulo de visión
}

pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub texture: Texture2D,
}

impl Entity {
    pub fn move_entity(&mut self, player_x: f32, player_y: f32) {
        let delta_y = player_y - self.y;
        let delta_x = player_x - self.x;
        let angle = delta_y.atan2(delta_x);

        self.angle = angle;
        
        let new_x = self.x + self.angle.cos() * 0.03;
        let new_y = self.y + self.angle.sin() * 0.03;

        let map = map::get_map();
        // Verificar colisiones
        if map[new_y as usize][new_x as usize] <= 0 {
            self.x = new_x;
            self.y = new_y;
        }
    }
}
