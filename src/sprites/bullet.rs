use crate::game_objects::direction::VerticalDirection;
use crate::game_objects::coordinate::Coordinate;

pub struct Bullet {
    game_scale: i32,
    current_location: Coordinate,
    screen_segments: Vec<Coordinate>,
    max_window_x_size: i32,
    direction: VerticalDirection
}

impl Bullet {
    pub fn new(game_scale: i32, current_location: Coordinate, max_window_x_size:i32, direction:VerticalDirection) -> Self {
        Self { game_scale: game_scale,
               max_window_x_size: max_window_x_size,
               screen_segments:
        vec![Coordinate::new(game_scale * 0 + &current_location.x, game_scale * 0 + &current_location.y), Coordinate::new(game_scale * 0 + &current_location.x, game_scale * 1 + &current_location.y),
                Coordinate::new(game_scale * 0 + &current_location.x, game_scale * 2 + &current_location.y), Coordinate::new(game_scale * 0 + &current_location.x, game_scale * 3 + &current_location.y),
                Coordinate::new(game_scale * 0 + &current_location.x, game_scale * 4 + &current_location.y), Coordinate::new(game_scale * 0 + &current_location.x, game_scale * 5 + &current_location.y)],
            current_location: current_location,
            direction: direction,
        }
    }
}

pub trait Projectile {
    fn move_object(&mut self);
    fn get_screen_segments(&self) -> &Vec<Coordinate>;
    fn colliding_with_index(&self, coordinates: Vec<Coordinate>) -> Option<i32>;
}

impl Projectile for Bullet {
    fn move_object(&mut self) {
        for i in 0..self.screen_segments.len() {
            self.screen_segments[i].y += (self.direction as i32) * 5;
        }
    }

    fn get_screen_segments(&self) -> &Vec<Coordinate> {
        &self.screen_segments
    }

    fn colliding_with_index(&self, coordinates: Vec<Coordinate>) -> Option<i32> {
        for i in 0..coordinates.len() {
            for j in 0..self.screen_segments.len() {
                if coordinates[i].x == self.screen_segments[j].x && coordinates[i].y == self.screen_segments[j].y {
                    return Some(i as i32);
                }
            }
        }
        return None;
    }
}
