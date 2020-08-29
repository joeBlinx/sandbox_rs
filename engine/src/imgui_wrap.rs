use imgui::{im_str, Condition, Window};
use imgui_sdl2;
use sdl2;
use std::rc::Rc;

pub struct ImguiWrap<'a> {
    imgui: imgui::Context,
    imgui_sdl2: imgui_sdl2::ImguiSdl2,
    renderer: imgui_opengl_renderer::Renderer,
    window: &'a sdl2::video::Window,
    items: Vec<Rc<dyn FnMut(&imgui::Ui)>>,
}

impl<'a> ImguiWrap<'a> {
    pub fn new(video_subsystem: &'a sdl2::VideoSubsystem, window: &'a sdl2::video::Window) -> Self {
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
        let imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);
        let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
            video_subsystem.gl_get_proc_address(s) as _
        });

        Self {
            imgui,
            imgui_sdl2,
            window,
            renderer,
            items: Vec::new(),
        }
    }

    pub fn handle_event(&mut self, event: &sdl2::event::Event) {
        self.imgui_sdl2.handle_event(&mut self.imgui, &event);
    }
    pub fn add_item(&mut self, item: Rc<dyn FnMut(&imgui::Ui)>) {
        self.items.push(item);
    }

    pub fn render(&mut self, mouse_state: &sdl2::mouse::MouseState) {
        self.imgui_sdl2
            .prepare_frame(self.imgui.io_mut(), self.window, mouse_state);
        let ui = self.imgui.frame();
        let items = &mut self.items;
        Window::new(im_str!("Hello world"))
            .size([300.0, 500.0], Condition::FirstUseEver)
            .build(&ui, || {
                for item in items.iter_mut() {
                    Rc::get_mut(item).unwrap()(&ui);
                }
            });
        self.imgui_sdl2.prepare_render(&ui, self.window);
        self.renderer.render(ui);
    }
}
