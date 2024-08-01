use crate::game_objects::coordinate::Coordinate;
use crate::sprites::sprites::Sprite;
use crate::game_objects::direction::HorizontalDirection;

pub struct Player {
    game_scale: i32,
    current_location: Coordinate,
    screen_segments: Vec<Coordinate>,
}

impl Player {
    pub fn new(game_scale: i32, current_location: Coordinate) -> Player {
        Player { game_scale: game_scale, current_location: current_location,
            screen_segments: 
        vec![Coordinate::new(self.game_scale * 0 + self.current_location.x, self.game_scale * 0 + self.current_location.y), Coordinate::new(self.game_scale * 0 + self.current_location.x, self.game_scale * 1 + self.current_location.y),
                Coordinate::new(self.game_scale * 0 + self.current_location.x, self.game_scale * 2 + self.current_location.y), Coordinate::new(self.game_scale * 0 + self.current_location.x, self.game_scale * 3 + self.current_location.y),
                Coordinate::new(self.game_scale * 0 + self.current_location.x, self.game_scale * 4 + self.current_location.y), Coordinate::new(self.game_scale * 0 + self.current_location.x, self.game_scale * 5 + self.current_location.y),
                Coordinate::new(self.game_scale * 0 + self.current_location.x, self.game_scale * 6 + self.current_location.y), 
                Coordinate::new(self.game_scale * 0 + self.current_location.x, self.game_scale * 7 + self.current_location.y), Coordinate::new(self.game_scale * 0 + self.current_location.x, self.game_scale * 8 + self.current_location.y),
                Coordinate::new(self.game_scale * 0 + self.current_location.x, self.game_scale * 9 + self.current_location.y), Coordinate::new(self.game_scale * 0 + self.current_location.x, self.game_scale * 10 + self.current_location.y)]
        }
    }
}

impl Sprite for Player {
    fn get_screen_segments(&self) -> Vec<Coordinate> {
        self.screen_segments.clone()
    }

    fn move_object(&mut self, direction: HorizontalDirection) {
        for i in 0..self.screen_segments.len() {
            self.screen_segments[i].x += (direction as i32) * 5;
        }
    }

    fn touching_horizontal_screen_edge(&self, right_edge: i32) -> HorizontalDirection {
        if self.screen_segments.iter().any(|elem| elem.y <= 0) {
            return HorizontalDirection::Left;
        }
        else if self.screen_segments.iter().any(|elem| elem.y >= right_edge) {
            return HorizontalDirection::Right;
        }
        return HorizontalDirection::Neither;
    }
}
