use imgui;
use imgui_opengl_renderer;
use imgui_sdl2;
use std::sync::{Mutex};

pub struct ImGuiInfo{
   // pub context: Mutex<imgui::Context>,
    pub imgui_sdl2: imgui_sdl2::ImguiSdl2,
    pub renderer: imgui_opengl_renderer::Renderer
}