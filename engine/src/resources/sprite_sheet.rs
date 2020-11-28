use std::collections::HashMap;

pub(crate) struct Position{
    pub(crate) x: f32,
    pub(crate) y: f32
}
pub(crate) struct Size{
    pub(crate) w: f32,
    pub(crate) h: f32
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
