extern crate opengl_graphics;
extern crate graphics;
extern crate piston;
extern crate glutin_window;

use self::opengl_graphics::{GlGraphics, OpenGL};
use self::piston::input::*;
use self::glutin_window::GlutinWindow as Window;
use self::piston::window::WindowSettings;
use self::piston::event_loop::*;

pub enum HoleVariant {
    Red,
    Green,
    Yellow,
}

pub struct Hole {
    position: (f64, f64),
    radius: f64,
    variant: HoleVariant,
}

pub struct App {
    gl: GlGraphics,
    holes: [Hole; 1],
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use self::graphics::*;

        const WHITE: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        let (circle_x, circle_y) = self.holes[0].position;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);
            let circle = ellipse::circle(25.0, 25.0, 10.0);
            let transform = c.transform.trans(x, y).trans(circle_x, circle_y);

            ellipse(RED, circle, transform, gl);
        });
    }
}

pub fn start() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("circles", [200, 200])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        holes: [
            Hole {
                position: (0.0, 0.0),
                radius: 2.0,
                variant: HoleVariant::Red,
            },
        ],
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
    }
}