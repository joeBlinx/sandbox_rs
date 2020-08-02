use imgui::sys::igGetWindowPos_nonUDT;

pub struct Imgui<'a>{
    imgui: imgui::Context,
    imgui_sdl2:imgui_sdl2::ImguiSdl2,
    renderer: imgui_opengl_renderer::Renderer,
    window_sdl2: &'a sdl2::video::Window,
    video : &'a sdl2::VideoSubsystem,

}


impl Imgui<'a>{
    pub fn new (window: &sdl2::video::Window, video: &sdl2::VideoSubsystem) -> Imgui<'a>{
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
        let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, window);
        let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);


        Imgui{
            imgui,
            imgui_sdl2,
            renderer,
            window_sdl2:window,
            video
        }
    }
    pub fn handle_event(&mut self, event: &sdl2::event::Event){
        self.imgui_sdl2.handle_event(&mut self.imgui, event);
    }

    pub fn frame(&mut self) -> imgui::Ui{
        self.imgui.frame()
    }

    pub fn render(&mut self, ui: imgui::Ui) {
        self.imgui_sdl2.prepare_render(&ui, self.window_sdl2);
        self.renderer.render(ui);
    }
}
