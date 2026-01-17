pub struct Layer {
    pub tiles: Vec<Vec<u32>>,
    pub width: usize,
    pub height: usize,
}

impl Layer {
    pub fn new(width: usize, height: usize, tiles: Vec<Vec<u32>>) -> Self {
        Self {
            tiles,
            width,
            height,
        }
    }
}

pub trait LayerDrawable {
    fn get_tiles(&self) -> Vec<Vec<u32>>;
}
