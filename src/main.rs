mod game;
mod player;
mod ball;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{ButtonEvent};

use player::{Player};
use game::{Game};
use ball::{Ball};

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Pong!", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        x: 0.0,
        y: 0.0,
        player1: Player{x: 0.0, y: 0.0, score: 0},
        player2: Player{x: 0.0, y: 0.0, score: 0},
        ball: Ball{x: 0.0, y: 0.0, velocityX: 2.0, velocityY: 2.0},
    };

    game.init();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(k) = e.button_args() {
            game.handle_input(&k);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }
    }
}