use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::video::WindowContext;

pub struct Window {
    
}

impl Window {
    pub fn new() -> Self {
        

        Self {
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            texture_creator,
        }
    }
}
