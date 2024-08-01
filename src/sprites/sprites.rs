use crate::game_objects::coordinate::Coordinate;
use crate::game_objects::direction::{HorizontalDirection, VerticalDirection};

pub trait Sprite {
    fn move_object(&mut self, direction: HorizontalDirection);
    fn get_screen_segments(&self) -> Vec<Coordinate>;
    fn touching_horizontal_screen_edge(&self, right_edge: i32) -> HorizontalDirection;
}

pub trait EnemySprite {
    fn alive(&self) -> bool;
    fn touching_bottom_screen_edge(&self) -> bool;
    fn move_object(&mut self, direction: VerticalDirection);
}
