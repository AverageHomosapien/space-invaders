use crate::game_objects::coordinate::Coordinate;
use crate::sprites::sprites::{Sprite, EnemySprite};
use crate::game_objects::direction::{HorizontalDirection, VerticalDirection};
use crate::game_objects::colors::{WHITE, BLACK};

pub struct BasicEnemy {
    game_scale: i32,
    current_location: Coordinate,
    screen_segments: Vec<Coordinate>,
    max_window_x_size: i32,
    color: colors,
}

impl BasicEnemy {
    pub fn new(game_scale: i32, current_location: Coordinate, color:color, max_window_x_size:i32) -> Self {
        Self { game_scale: game_scale,
               max_window_x_size: max_window_x_size,
               screen_segments: 
        vec![Coordinate::new(game_scale * 2 + &current_location.x, 0 + &current_location.y), Coordinate::new(game_scale * 8 + &current_location.x, 0 + &current_location.y),
             Coordinate::new(game_scale * 3 + &current_location.x, game_scale * 1 + &current_location.y), Coordinate::new(game_scale * 7 + &current_location.x, game_scale * 1 + &current_location.y),
             Coordinate::new(game_scale * 2 + &current_location.x, game_scale * 2 + &current_location.y),Coordinate::new(game_scale * 3 + &current_location.x, game_scale * 2 + &current_location.y), 
                Coordinate::new(game_scale * 4 + &current_location.x, game_scale * 2 + &current_location.y), Coordinate::new(game_scale * 5 + &current_location.x, game_scale * 2 + &current_location.y),
                Coordinate::new(game_scale * 6 + &current_location.x, game_scale * 2 + &current_location.y), Coordinate::new(game_scale * 7 + &current_location.x, game_scale * 2 + &current_location.y),
                Coordinate::new(game_scale * 8 + &current_location.x, game_scale * 2 + &current_location.y),
            Coordinate::new(game_scale * 1 + &current_location.x, game_scale * 3 + &current_location.y),Coordinate::new(game_scale * 2 + &current_location.x, game_scale * 3 + &current_location.y), 
                Coordinate::new(game_scale * 4 + &current_location.x, game_scale * 3 + &current_location.y), Coordinate::new(game_scale * 5 + &current_location.x, game_scale * 3 + &current_location.y),
                Coordinate::new(game_scale * 6 + &current_location.x, game_scale * 3 + &current_location.y), Coordinate::new(game_scale * 8 + &current_location.x, game_scale * 3 + &current_location.y),
                Coordinate::new(game_scale * 9 + &current_location.x, game_scale * 3 + &current_location.y),
            Coordinate::new(game_scale * 0 + &current_location.x, game_scale * 4 + &current_location.y), Coordinate::new(game_scale * 1 + &current_location.x, game_scale * 4 + &current_location.y),
                Coordinate::new(game_scale * 2 + &current_location.x, game_scale * 4 + &current_location.y), Coordinate::new(game_scale * 3 + &current_location.x, game_scale * 4 + &current_location.y),
                Coordinate::new(game_scale * 4 + &current_location.x, game_scale * 4 + &current_location.y), Coordinate::new(game_scale * 5 + &current_location.x, game_scale * 4 + &current_location.y),
                Coordinate::new(game_scale * 6 + &current_location.x, game_scale * 4 + &current_location.y), 
                Coordinate::new(game_scale * 7 + &current_location.x, game_scale * 4 + &current_location.y), Coordinate::new(game_scale * 8 + &current_location.x, game_scale * 4 + &current_location.y),
                Coordinate::new(game_scale * 9 + &current_location.x, game_scale * 4 + &current_location.y), Coordinate::new(game_scale * 10 + &current_location.x, game_scale * 4 + &current_location.y),
            Coordinate::new(game_scale * 0 + &current_location.x, game_scale * 5 + &current_location.y),
                Coordinate::new(game_scale * 2 + &current_location.x, game_scale * 5 + &current_location.y), Coordinate::new(game_scale * 3 + &current_location.x, game_scale * 5 + &current_location.y),
                Coordinate::new(game_scale * 4 + &current_location.x, game_scale * 5 + &current_location.y), Coordinate::new(game_scale * 5 + &current_location.x, game_scale * 5 + &current_location.y),
                Coordinate::new(game_scale * 6 + &current_location.x, game_scale * 5 + &current_location.y), 
                Coordinate::new(game_scale * 7 + &current_location.x, game_scale * 5 + &current_location.y), Coordinate::new(game_scale * 8 + &current_location.x, game_scale * 5 + &current_location.y),
                Coordinate::new(game_scale * 10 + &current_location.x, game_scale * 5 + &current_location.y),
            Coordinate::new(game_scale * 0 + &current_location.x, game_scale * 6 + &current_location.y), Coordinate::new(game_scale * 2 + &current_location.x, game_scale * 6 + &current_location.y), 
                Coordinate::new(game_scale * 8 + &current_location.x, game_scale * 6 + &current_location.y), Coordinate::new(game_scale * 10 + &current_location.x, game_scale * 6 + &current_location.y),
            Coordinate::new(game_scale * 2 + &current_location.x, game_scale * 7 + &current_location.y), Coordinate::new(game_scale * 3 + &current_location.x, game_scale * 7 + &current_location.y),
                Coordinate::new(game_scale * 7 + &current_location.x, game_scale * 7 + &current_location.y), Coordinate::new(game_scale * 8 + &current_location.x, game_scale * 7 + &current_location.y)],
            current_location: current_location,
            color: color
        }
    }
}

impl Sprite for BasicEnemy {
    fn get_screen_segments(&self) -> &Vec<Coordinate> {
        &self.screen_segments
    }

    fn move_object(&mut self, direction: HorizontalDirection) {
        for i in 0..self.screen_segments.len() {
            self.screen_segments[i].x += HorizontalDirection::Right as i32 * 5;
            if self.screen_segments[i].x >= self.max_window_x_size as i32 {
                self.screen_segments[i].x = 0;
                self.screen_segments[i].y += 20;
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

impl EnemySprite for BasicEnemy {
    fn alive(&self) -> bool {
        !self.screen_segments.is_empty()
    }

    fn touching_bottom_screen_edge(&self) -> bool {
        self.screen_segments.iter().any(|elem| elem.x <= 0)
    }

    fn move_object(&mut self, direction: VerticalDirection) {
        for i in 0..self.screen_segments.len() {
            self.screen_segments[i].y += (direction as i32) * 5;
        }
    }
}
