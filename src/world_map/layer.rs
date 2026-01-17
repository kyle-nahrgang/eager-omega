use macroquad::math::Vec2;

pub trait Layer {
    fn is_collision(&self, position: Vec2, size: Vec2) -> bool;
    fn get_tile(&self, x: usize, y: usize) -> Option<u32>;
}
