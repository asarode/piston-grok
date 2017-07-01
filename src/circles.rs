extern crate opengl_graphics;
extern crate graphics;
extern crate piston;
extern crate glutin_window;

use self::opengl_graphics::{GlGraphics, OpenGL};
use self::piston::input::*;
use self::glutin_window::{GlutinWindow as Window, OpenGL as GlutinOpenGL};
use self::piston::window::WindowSettings;
use self::piston::event_loop::*;

pub enum HoleVariant {
    Red,
    Green,
    Blue,
}

pub struct Hole {
    position: (f64, f64),
    radius: f64,
    variant: HoleVariant,
}

pub struct App {
    gl: GlGraphics,
    holes: [Hole; 3],
}

impl App {
    fn new(opengl: GlutinOpenGL) -> App {
        let HOLE_SEPARATION = 200.0;
        let HOLE_SIZE = 70.0;
        App {
            gl: GlGraphics::new(opengl),
            holes: [
                Hole {
                    position: (0.0, HOLE_SEPARATION),
                    radius: HOLE_SIZE,
                    variant: HoleVariant::Red,
                },
                Hole {
                    position: (0.87 * HOLE_SEPARATION, -0.5 * HOLE_SEPARATION),
                    radius: HOLE_SIZE,
                    variant: HoleVariant::Blue,
                },
                Hole {
                    position: (-0.87 * HOLE_SEPARATION, -0.5 * HOLE_SEPARATION),
                    radius: HOLE_SIZE,
                    variant: HoleVariant::Green,
                },
            ],
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use self::graphics::*;

        const WHITE: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        let ref holes = self.holes;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);

            for hole in holes.iter() {
                let (circle_x, circle_y) = hole.position;
                let circle = ellipse::circle(circle_x, circle_y, hole.radius);
                let transform = c.transform.trans(x, y);
                let color = match hole.variant {
                    HoleVariant::Red => RED,
                    HoleVariant::Green => GREEN,
                    HoleVariant::Blue => BLUE,
                };

                ellipse(color, circle, transform, gl);

            }
        });
    }
}

pub fn start() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("circles", [800, 800])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
    }
}
