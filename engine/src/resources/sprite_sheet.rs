use std::collections::HashMap;

pub(crate) struct Position{
    pub(crate) x: i32,
    pub(crate) y: i32
}
pub(crate) struct Size{
    pub(crate) w: i32,
    pub(crate) h: i32
}
pub(crate) struct Sprite {
    pub(crate) frame: (Position, Size),
    pub(crate) rotated: bool,
    pub(crate) trimmed: bool,
    pub(crate) sprite_source_size:(Position, Size),
    pub(crate) source_size: Size,
    pub(crate) duration:i32,
}
pub struct Animation{
    pub(crate) from: i32,
    pub(crate) to: i32,
    pub(crate) direction: String
}
pub struct SpriteSheet{
    pub(crate) image: String,
    pub(crate) format: String,
    pub(crate) size: Size,
    pub(crate) scale: i32,
    pub(crate) animation: HashMap<String, Animation>,
    pub(crate) sprites: Vec<Sprite>
}
impl Default for SpriteSheet{
    fn default() -> Self {
        Self{
            image: String::from(""),
            format: String::from(""),
            size: Size{w:0, h:0},
            scale: 1,
            animation: HashMap::default(),
            sprites: Vec::default(),

        }
    }
}