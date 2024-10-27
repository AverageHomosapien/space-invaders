use crate::game_objects::coordinate::Coordinate;
use crate::game_objects::direction::{HorizontalDirection, VerticalDirection};

pub trait Bullet {
    fn move_object(&mut self, direction: VerticalDirection);
    fn get_screen_segments(&self) -> &Vec<Coordinate>;
    fn colliding_with_index(&self, coordinates: Vec<Coordinate>) -> Some(i32);
}

pub trait Sprite {
    fn move_object(&mut self, direction: HorizontalDirection);
    fn get_screen_segments(&self) -> &Vec<Coordinate>;
    fn touching_horizontal_screen_edge(&self) -> HorizontalDirection;
}

pub trait EnemySprite {
    fn alive(&self) -> bool;
    fn touching_bottom_screen_edge(&self) -> bool;
    fn move_object(&mut self, direction: VerticalDirection);
}
