use crate::game_objects::coordinate::Coordinate;
use crate::sprites::sprites::Sprite;
use crate::game_objects::direction::HorizontalDirection;

pub struct Player {
    game_scale: i32,
    current_location: Coordinate,
    screen_segments: Vec<Coordinate>,
    max_window_x_size: i32,
}

impl Player {
    pub fn new(game_scale: i32, current_location: Coordinate, max_window_x_size:i32) -> Self {
        Self { game_scale: game_scale,
               max_window_x_size: max_window_x_size,
               screen_segments:
        vec![Coordinate::new(game_scale * 0 + &current_location.x, game_scale * 0 + &current_location.y), Coordinate::new(game_scale * 1 + &current_location.x, game_scale * 0 + &current_location.y),
                Coordinate::new(game_scale * 2 + &current_location.x, game_scale * 0 + &current_location.y), Coordinate::new(game_scale * 3 + &current_location.x, game_scale * 0 + &current_location.y),
                Coordinate::new(game_scale * 4 + &current_location.x, game_scale * 0 + &current_location.y), Coordinate::new(game_scale * 5 + &current_location.x, game_scale * 0 + &current_location.y),
                Coordinate::new(game_scale * 6 + &current_location.x, game_scale * 0 + &current_location.y), 
                Coordinate::new(game_scale * 7 + &current_location.x, game_scale * 0 + &current_location.y), Coordinate::new(game_scale * 8 + &current_location.x, game_scale * 0 + &current_location.y),
                Coordinate::new(game_scale * 9 + &current_location.x, game_scale * 0 + &current_location.y), Coordinate::new(game_scale * 10 + &current_location.x, game_scale * 0 + &current_location.y)],
            current_location: current_location,
        }
    }
}

impl Sprite for Player {
    fn get_screen_segments(&self) -> &Vec<Coordinate> {
        &self.screen_segments
    }

    fn move_object(&mut self, direction: HorizontalDirection) {
        if let direction != HorizontalDirection::Neither 
            && direction != self.touching_horizontal_screen_edge() {
            for i in 0..self.screen_segments.len() {
                self.screen_segments[i].x += (direction as i32) * 5;
            }
        }
    }

    fn touching_horizontal_screen_edge(&self) -> HorizontalDirection {
        if self.screen_segments.iter().any(|elem| elem.y <= 0) {
            return HorizontalDirection::Left;
        }
        else if self.screen_segments.iter().any(|elem| elem.y >= max_window_x_size) {
            return HorizontalDirection::Right;
        }
        return HorizontalDirection::Neither;
    }
}
