pub trait HandleEvent {
    fn handle_event(&mut self, event: &sdl2::event::Event);
}
