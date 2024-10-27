extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use crate::piston::EventLoop;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, ButtonArgs, ButtonEvent, Button, ButtonState, Key};
use piston::window::WindowSettings;
use graphics::*;
use crate::game_objects::colors::{WHITE, BLACK};
use crate::sprites::sprite_factory::{ SpriteFactory, EnemyFactory, PlayerFactory } ;
use crate::game_objects::game_state::GameState;
use crate::game_controller::GameController;
use crate::game_objects::coordinate::Coordinate;

pub mod game_objects;
pub mod sprites;

const GAME_SCALE: i32 = 10;
const WINDOW_X: u32 = 700;
const WINDOW_Y: u32 = 700;

pub struct App {
    gl: GlGraphics,
    game_controller: GameController,
}

impl App {
    // Logic for rendering all objects on the screen. Called on each frame
    fn render(&mut self, args: &RenderArgs){
        self.game_controller.render_objects(&self.gl);
    }

    // Logic for updating all objects
    fn update(&mut self, args: &UpdateArgs) {
        self.game_controller.update_on_game_frame();
    }
}

fn main(){
    let open_gl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Space Invaders", [WINDOW_X, WINDOW_Y])
        .graphics_api(open_gl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let enemy_starting_coords = vec![Coordinate::new(30,0), Coordinate::new(160, 0), Coordinate::new(290, 0), Coordinate::new(420, 0), Coordinate::new(550, 0), Coordinate::new(30, 100), Coordinate::new(160, 100), Coordinate::new(290, 100), Coordinate::new(420, 100), Coordinate::new(550, 100), Coordinate::new(30, 200), Coordinate::new(160, 200), Coordinate::new(290, 200), Coordinate::new(420, 200), Coordinate::new(550, 200)];

    let sprite_factory = SpriteFactory::new(GAME_SCALE,  WINDOW_X as i32);
    let enemies = sprite_factory.get_basic_enemies(enemy_starting_coords, WHITE);
    let player = sprite_factory.get_player(Coordinate::new(WINDOW_X as i32 / 2, WINDOW_Y as i32 - 20));

    let controller = GameController();

    let mut app = App {
        gl: GlGraphics::new(open_gl),
        game_controller: controller,
    };

    let event_settings = EventSettings::new().ups(15);
    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(&mut window) {

        // Render loop
        if let Some(args) = e.render_args(){
            app.render(&args);
        }

        // Run the update loop
        if let Some(args) = e.update_args(){
            app.update(&args);
        }

        // If player pressing a key
        if let Some(args) = e.button_args() {
            
        }

        match app.game_controller.get_game_state() {
            GameState::Lost => {
                println!("Game over buster. Your score is: {}", app.game_controller.score);
                return;
            },
            GameState::Won => {
                println!("Game over... You won!! Your score is: {}", app.game_controller.score);
                return;
            },
            _ => continue
        }
    }
}

fn round_to_game_scale(n: i32) -> i32{
    let a = (n/GAME_SCALE) * GAME_SCALE as i32;
    let b = a + GAME_SCALE;
    if n - a > b - n {
        return b;
    }
    return a;
}
