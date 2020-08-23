use imgui_sdl2;
use sdl2;
pub struct ImguiWrap<'a>{
    imgui : imgui::Context,
    imgui_sdl2 :imgui_sdl2::ImguiSdl2,
    renderer: imgui_opengl_renderer::Renderer,
    window: &'a sdl2::video::Window,
}

impl<'a> ImguiWrap<'a>{
    pub fn new (video_subsystem:&'a sdl2::VideoSubsystem, window: &'a sdl2::video::Window) -> Self{

        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
        let imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);
        let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video_subsystem.gl_get_proc_address(s) as _);

        Self{
            imgui, 
            imgui_sdl2,
            window, 
            renderer
        }
    }

    pub fn handle_event(&mut self, event: &sdl2::event::Event){
        self.imgui_sdl2.handle_event(&mut self.imgui, &event);
    }

    pub fn render<T: FnOnce(&imgui::Ui)>(&mut self, mouse_state : &sdl2::mouse::MouseState, function : T){

        self.imgui_sdl2.prepare_frame(self.imgui.io_mut(), self.window, mouse_state);
        let ui = self.imgui.frame();
        function(&ui);
        self.imgui_sdl2.prepare_render(&ui, self.window);
        self.renderer.render(ui);
    }
}