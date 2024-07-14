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
use rand::Rng;

enum Direction {
    Left, Right
}

pub struct Segment {
    x: i32,
    y: i32,
}

pub struct App {
    gl: GlGraphics,
    score: u32,
    direction: Direction,
    gameover: bool,
    enemies: Vec<Vec<Segment>>,
}

impl App {
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];i
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        self.gl.draw(args.viewport(), |c, gl|{
            clear(BLACK, gl);
            let transform = c.transform.trans(0.0,0.0).rot_deg(0.0);
            for i in &self.enemies[0] {
                let printable_segment = rectangle::square(i.x as f64, i.y as f64, 10.0);
                rectangle(WHITE, printable_segment, transform, gl);
            }
        });
    }
}

fn main(){
    let open_gl = OpenGL::V3_2;
    let window_x: u32 = 500;
    let window_y: u32 = 500;

    let mut window: Window = WindowSettings::new("Space Invaders", [window_x, window_y])
        .graphics_api(open_gl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(open_gl),
        score: 0,
        direction: Direction::Right,
        gameover: false,
        enemies: vec![create_enemy_segments(0,0)],
    };

    let event_settings = EventSettings::new().ups(15);
    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(&mut window){
        if let Some(args) = e.render_args(){
            app.render(&args);
        }

        if let Some(args) = e.update_args(){
        }
        //if let app.gameover {
        //    println!("Game over buster. Your score is: {}", app.score);
        //    return;
        //}
    }
}

fn round_to_nearest_10(n: i32) -> i32{
    let a = (n/10) * 10 as i32;
    let b = a + 10;
    if n - a > b - n {
        return b;
    }
    return a;
}

fn create_enemy_segments(position_x: i32, position_y: i32) -> Vec<Segment> {
    vec![Segment{x:0 + position_x, y:0}, Segment {x: 10 + position_x, y:10}, Segment {x:20 + position_x, y:0}]
}
