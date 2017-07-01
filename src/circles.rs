extern crate opengl_graphics;
extern crate graphics;
extern crate piston;
extern crate glutin_window;

use self::opengl_graphics::{GlGraphics, OpenGL};
use self::piston::input::*;
use self::glutin_window::{GlutinWindow as Window, OpenGL as GlutinOpenGL};
use self::piston::window::WindowSettings;
use self::piston::event_loop::*;

const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const NEUTRAL_SIMON: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub enum HoleVariant {
    Red,
    Green,
    Blue,
    Neutral,
}

pub struct Hole {
    position: (f64, f64),
    radius: f64,
    variant: HoleVariant,
}

pub struct App {
    gl: GlGraphics,
    holes: [Hole; 3],
    simon: Hole,
}

impl App {
    fn new(opengl: GlutinOpenGL) -> App {
        let hole_separation = 200.0;
        let hole_size = 70.0;
        App {
            gl: GlGraphics::new(opengl),
            holes: [
                Hole {
                    position: (0.0, hole_separation),
                    radius: hole_size,
                    variant: HoleVariant::Red,
                },
                Hole {
                    position: (0.87 * hole_separation, -0.5 * hole_separation),
                    radius: hole_size,
                    variant: HoleVariant::Blue,
                },
                Hole {
                    position: (-0.87 * hole_separation, -0.5 * hole_separation),
                    radius: hole_size,
                    variant: HoleVariant::Green,
                },
            ],
            simon: Hole {
                position: (0.0, 0.0),
                radius: 25.0,
                variant: HoleVariant::Neutral,
            },
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use self::graphics::*;

        let (viewport_x, viewport_y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        let ref holes = self.holes;
        let ref simon = self.simon;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);

            let (simon_x, simon_y) = simon.position;
            let simon_circle = ellipse::circle(simon_x, simon_y, simon.radius);
            let simon_transform = c.transform.trans(viewport_x, viewport_y);
            let simon_color = color_from_variant(&simon.variant);

            ellipse(simon_color, simon_circle, simon_transform, gl);


            for hole in holes.iter() {
                let (circle_x, circle_y) = hole.position;
                let circle = ellipse::circle(circle_x, circle_y, hole.radius);
                let transform = c.transform.trans(viewport_x, viewport_y);
                let color = color_from_variant(&hole.variant);

                ellipse(color, circle, transform, gl);

            }
        });
    }
}

fn color_from_variant(variant: &HoleVariant) -> [f32; 4] {
    match *variant {
        HoleVariant::Red => RED,
        HoleVariant::Green => GREEN,
        HoleVariant::Blue => BLUE,
        HoleVariant::Neutral => NEUTRAL_SIMON,
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
