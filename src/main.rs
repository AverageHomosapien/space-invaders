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
use crate::game_objects::coordinate::Coordinate;
use crate::sprites::enemies::{BasicEnemy, Enemy};
use graphics::*;
use crate::game_objects::colors::{WHITE, BLACK};

pub mod game_objects;
pub mod sprites;

const game_scale: u32 = 10;

enum Direction {
    Left, Right
}

pub struct App {
    gl: GlGraphics,
    score: u32,
    direction: Direction,
    gameover: bool,
    enemies: Vec<Vec<Coordinate>>,
}

impl App {
    fn render(&mut self, args: &RenderArgs){
        self.gl.draw(args.viewport(), |c, gl|{
            clear(BLACK, gl);
            let transform = c.transform.trans(0.0,0.0).rot_deg(0.0);
            for enemy in &self.enemies {
                for pixel in enemy {
                    let printable_segment = rectangle::square(pixel.y as f64, pixel.x as f64, 10.0);
                    rectangle(WHITE, printable_segment, transform, gl);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.score += 1;
        if self.score % 1 == 0 {
            for i in 0..self.enemies.len() {
                for n in 0..self.enemies[i].len() {
                    self.enemies[i][n].y += 1;
                }
            }
        }
    }
}

fn main(){
    let open_gl = OpenGL::V3_2;
    let window_x: u32 = 700;
    let window_y: u32 = 500;

    let mut window: Window = WindowSettings::new("Space Invaders", [window_x, window_y])
        .graphics_api(open_gl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let enemy_starting_coords = vec![Coordinate::new(0,0), Coordinate::new(0, 130), Coordinate::new(0, 260), Coordinate::new(0, 390)];
    let basic_enemy = BasicEnemy { game_scale: game_scale };
    let mut enemies: Vec<Vec<Coordinate>> = vec![vec![]];

    for coord in enemy_starting_coords {
        enemies.push(basic_enemy.get_screen_segments(coord));
    }

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

fn round_to_game_scale(n: u32) -> u32{
    let a = (n/game_scale) * game_scale as u32;
    let b = a + game_scale;
    if n - a > b - n {
        return b;
    }
    return a;
}
