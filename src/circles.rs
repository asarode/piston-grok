extern crate opengl_graphics;
extern crate graphics;

use self::opengl_graphics::GlGraphics;
use self::graphics::math::Matrix2d;

pub enum HoleVariant {
    Red,
    Green,
    Yellow,
}

pub struct Hole {
    position: Matrix2d,
    radius: f64,
    variant: HoleVariant,
}

pub struct App {
    gl: GlGraphics,
    holes: [Hole; 3],
}