use opengl_graphics::{GlGraphics};
use piston::input::{RenderArgs, UpdateArgs, ButtonArgs, Button, ButtonState, Key};

use player::{Player};
use ball::{Ball};

pub struct Game {
    pub gl: GlGraphics,
    pub x: f64,
    pub y: f64,
    pub player1: Player,
    pub player2: Player,
    pub ball: Ball,
}

impl Game {
    pub fn init(&mut self) {
        self.player1.x = 50.0;
        self.player1.y = 100.0;

        self.player2.x = 750.0;
        self.player2.y = 200.0;

        self.ball.x = 390.0;
        self.ball.y = 290.0;
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        println!("Updating Game");
        println!("Ball: {}:{}", self.ball.x, self.ball.y);
        if (self.ball.x < 500.0 && self.ball.y < 700.0)  {
            self.ball.x += 5.0;
        } 
    }

    pub fn handle_collisions(self, x: f64, y: f64) {
        // if ()
    }


    pub fn handle_input(&mut self, k: &ButtonArgs) {
        if k.state == ButtonState::Press {
            match k.button {
                Button::Keyboard(Key::Right) => {
                    self.player1.x += 10.0;
                },
                Button::Keyboard(Key::Left) => {
                    self.player1.x -= 10.0;
                },
                Button::Keyboard(Key::Up) => {
                    self.player1.y -= 10.0;
                },
                Button::Keyboard(Key::Down) => {
                    self.player1.y += 10.0;
                },
                _ => println!("Another Key Pressed"),
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [243.0, 123.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
        });

        self.ball.render(&mut self.gl, args);
        self.player1.render(&mut self.gl, args);
        self.player2.render(&mut self.gl, args);
    }
}