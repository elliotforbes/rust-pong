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
        self.player2.x = 100.0;
        self.player2.y = 100.0;
    }


    pub fn update(&mut self, _args: &UpdateArgs) {
        println!("Updating Game");

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

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
        });

        self.ball.render(&mut self.gl, args);
        self.player1.render(&mut self.gl, args);
        self.player2.render(&mut self.gl, args);
    }
}