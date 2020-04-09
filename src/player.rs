use opengl_graphics::{GlGraphics};
use piston::input::{RenderArgs};

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub x: f64,
    pub y: f64,
    pub score: i32,
}

impl Player {
    pub fn update(&mut self) {
        println!("Moving Player");
    }

    pub fn score(self) {
        println!("Score");
    }

    pub fn render(self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        gl.draw(args.viewport(), |c, gl| {

            let square = rectangle::square(0.0, 0.0, 50.0);

            let transform = c
                .transform
                .trans(self.x, self.y)
                .rot_rad(0.0)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });

    }
}