use macroquad::texture::Texture2D;

pub mod board;
mod pieces;
pub mod input;

pub struct Renderer<'a> {
    texture: &'a Texture2D
}

impl Renderer<'_> {
    pub fn new(texture: &Texture2D) -> Renderer<'_> {
        Renderer {
            texture
        }
    }
}
