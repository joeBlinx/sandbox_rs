use std::path::Path;
use std::{fs, fmt};
use std::io;
use json::JsonValue;
use crate::resources::sprite_sheet::{SpriteSheet, Sprite, Position, Size, Animation};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub enum SpriteError{
    IoErr(io::Error),
    JsonErr(json::Error),
}
impl Debug for SpriteError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self{
            SpriteError::IoErr(err) => err.fmt(f),
            SpriteError::JsonErr(err) => err.fmt(f)
        }
    }
}
pub fn read_sprite_sheet(json_path: &Path) -> Result<SpriteSheet, SpriteError>{
    let file_content;
    match  fs::read_to_string(json_path){
        Ok(value) => file_content = value,
        Err(io_err) => {return Err(SpriteError::IoErr(io_err))}
    }
    let parsed;
    match  json::parse(&file_content){
        Ok(value) => parsed = value,
        Err(json_err) => {return Err(SpriteError::JsonErr(json_err))}
    }

    let frames = &parsed["frames"];
    let fill_frame = |sprite:&JsonValue|{
        let frame = &sprite["frame"];
        let sprite_source_size = &sprite["spriteSourceSize"];
        let source_size = &sprite["sourceSize"];
        Sprite {
            frame:(Position{x:frame["x"].as_f32().unwrap(),
                y:frame["y"].as_f32().unwrap()},
                   Size{w:frame["w"].as_f32().unwrap(),
                       h:frame["h"].as_f32().unwrap()}),
            rotated:sprite["rotated"].as_bool().unwrap(),
            trimmed:sprite["trimmed"].as_bool().unwrap(),
            sprite_source_size: (Position{x:sprite_source_size["x"].as_f32().unwrap(),
                                y:sprite_source_size["y"].as_f32().unwrap()},
                                 Size{w:sprite_source_size["w"].as_f32().unwrap(),
                                     h:sprite_source_size["h"].as_f32().unwrap()}),
            source_size : Size{w:source_size["w"].as_f32().unwrap(), h:source_size["h"].as_f32().unwrap()},
            duration:sprite["duration"].as_i32().unwrap()
        }
    };
    let mut sprites = Vec::with_capacity(frames.len());
    for (_, sprite) in frames.entries(){
        sprites.push(fill_frame(sprite));
    }
    let meta = &parsed["meta"];
    let size = &meta["size"];
    let fill_frametags= |frame_tags:&JsonValue|{
        let mut animations = HashMap::with_capacity(frame_tags.len());
        for (_, frame_tag) in frame_tags.entries(){
            let animation_name = frame_tag["name"].as_str().unwrap().to_owned();
            animations.insert(animation_name,
                Animation{
                    from: frame_tag["from"].as_i32().unwrap(),
                    to: frame_tag["to"].as_i32().unwrap(),
                    direction: frame_tag["direction"].as_str().unwrap().to_owned()
                }
            );
        }
        animations
    };
    let mut sprite_sheet =
    SpriteSheet{
        image: meta["image"].as_str().unwrap().to_owned(),
        format: meta["format"].as_str().unwrap().to_owned(),
        size: Size{w:size["w"].as_f32().unwrap(),
            h:size["h"].as_f32().unwrap()},
        scale:meta["scale"].as_str().unwrap().parse::<i32>().unwrap(),
        animation: fill_frametags(&meta["frameTags"]),
        sprites
    };
    let size = &sprite_sheet.size;
    for sprite in sprite_sheet.sprites.iter_mut(){
        let mut frame = &mut sprite.frame;
        frame.0.x = frame.0.x / size.w;
        frame.0.y = frame.0.y / size.h;

        frame.1.w = frame.1.w / size.w;
        frame.1.h = frame.1.h / size.h;
    }
    Ok(sprite_sheet)
}