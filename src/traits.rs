use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
pub trait Drawable{
    fn render(&self, gl: &GlGraphics, args: &RenderArgs);
}