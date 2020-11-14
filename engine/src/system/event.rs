use legion::system;
use sdl2;
use crate::component::event::CloseEvent;

#[system(for_each)]
pub fn quit_event (close_event: &mut CloseEvent, #[resource]event: &sdl2::event::Event){
    match event{
        sdl2::event::Event::Quit {..} => close_event.event = true,
        sdl2::event::Event::KeyDown { keycode, .. } => {
            let keycode = keycode.unwrap();
            match keycode {
                sdl2::keyboard::Keycode::Escape => close_event.event = true,
                _ => {}
            }
        },
        _ => {}
    }
}