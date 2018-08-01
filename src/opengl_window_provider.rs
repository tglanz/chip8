use piston::window::WindowSettings;
use glutin_window::GlutinWindow;
use opengl_graphics::{ OpenGL };

use program::{WindowProvider};

/// TODO@TalG : Pass params such as scale, resolution etc ...
pub struct OpenGLWindowProvider {
}

impl OpenGLWindowProvider {
    pub fn new() -> OpenGLWindowProvider {
        OpenGLWindowProvider {

        }
    }
}

impl WindowProvider for OpenGLWindowProvider {
    fn into_window(self) -> GlutinWindow {
        let scale = 10;
        WindowSettings::new(
                "chip8",
                [64 * scale, 32 * scale]
            )
            .opengl(OpenGL::V3_2)
            .exit_on_esc(true)
            .build()
            .unwrap()
    }
}