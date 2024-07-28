use crate::game_objects::coordinate::Coordinate;
use crate::game_objects:direction::HorzontalDirection;

pub trait Sprite {
    fn move_object(&self, direction: HorizontalDirection);
    fn get_screen_segments(&self) -> Vec<Coordinate>;
    fn alive(&self) -> bool;
    fn touching_horizontal_screen_edge(&self, right_edge: i32) -> HorizontalDirection;
}

