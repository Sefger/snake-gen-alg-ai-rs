use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
pub trait Drawable{
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs);
}