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
//use rand::Rng;
//use crate::sprites;
use crate::game_objects::{coordinate::Coordinate, direction::Direction};
use crate::sprites::enemies::BasicEnemy;
use crate::sprites::player::Player;
use crate::sprites::sprites::Sprite;
use graphics::*;
use crate::game_objects::colors::{WHITE, BLACK};
use crate::sprites::sprite_factory::{ SpriteFactory, EnemyFactory } ;

pub mod game_objects;
pub mod sprites;

const GAME_SCALE: i32 = 10;
const WINDOW_X: u32 = 700;
const WINDOW_Y: u32 = 500;


pub struct App {
    gl: GlGraphics,
    score: i32,
    direction: Direction,
    gameover: bool,
    enemies: Vec<BasicEnemy>,
}

impl App {
    // All the draw logic exists in this loop
    fn render(&mut self, args: &RenderArgs){
        self.gl.draw(args.viewport(), |c, gl|{
            clear(BLACK, gl);
            let transform = c.transform.trans(0.0,0.0).rot_deg(0.0);
            for coord in &self.enemies.current_location {
                for pixel in coord {
                    let printable_segment = rectangle::square(pixel.y as f64, pixel.x as f64, 10.0);
                    rectangle(WHITE, printable_segment, transform, gl);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.score += 1;
        if self.score % 2 == 0 {

            if self.direction = Diction::Left && self.enemies.iter().any(|enemy| enemy.touching_horizontal_screen_edge(WINDOW_X) == Direction::Right) {
                self.direction = Direction::Right;
            }
            else if self.direction = Diction::Right && self.enemies.iter().any(|enemy| enemy.touching_horizontal_screen_edge() == Direction::Left) {
                self.direction = Direction::Left;
            }

            for i in 0..self.enemies.len() {
                self.enemies[i].move_object(self.direction);
            }

            for i in 0..self.enemies.len() {
                for n in 0..self.enemies[i].current_location.len() {
                    self.enemies.current_location[i][n].y += (self.direction as i32) * 5;
                }
            }
        }
    }
}

fn main(){
    let open_gl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Space Invaders", [window_x, window_y])
        .graphics_api(open_gl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let enemy_starting_coords = vec![Coordinate::new(0,0), Coordinate::new(0, 130), Coordinate::new(0, 260), Coordinate::new(0, 390)];

    let mut spriteFactory = SpriteFactory { game_scale: game_scale };
    let enemies = spriteFactory.get_basic_enemies(enemy_starting_coords);

    let mut app = App {
        gl: GlGraphics::new(open_gl),
        score: 0,
        direction: Direction::Right,
        gameover: false,
        enemies: enemies,
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

        if app.gameover {
            println!("Game over buster. Your score is: {}", app.score);
            return;
        }
    }
}

fn round_to_game_scale(n: i32) -> i32{
    let a = (n/game_scale) * game_scale as i32;
    let b = a + game_scale;
    if n - a > b - n {
        return b;
    }
    return a;
}
