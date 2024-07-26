use crate::game_objects::coordinate::Coordinate;
use crate::game_objects::sprites:Sprite;

pub struct Player {
    fn game_scale: i32,
}

impl Sprite for Player {
    fn get_screen_segments(&self, location: Coordinate) -> Vec<Coordinate> {
        vec![Coordinate::new(self.game_scale * 0 + location.x, self.game_scale * 0 + location.y), Coordinate::new(self.game_scale * 0 + location.x, self.game_scale * 1 + location.y),
                Coordinate::new(self.game_scale * 0 + location.x, self.game_scale * 2 + location.y), Coordinate::new(self.game_scale * 0 + location.x, self.game_scale * 3 + location.y),
                Coordinate::new(self.game_scale * 0 + location.x, self.game_scale * 4 + location.y), Coordinate::new(self.game_scale * 0 + location.x, self.game_scale * 5 + location.y),
                Coordinate::new(self.game_scale * 0 + location.x, self.game_scale * 6 + location.y), 
                Coordinate::new(self.game_scale * 0 + location.x, self.game_scale * 7 + location.y), Coordinate::new(self.game_scale * 4 + location.x, self.game_scale * 8 + location.y),
                Coordinate::new(self.game_scale * 4 + location.x, self.game_scale * 9 + location.y), Coordinate::new(self.game_scale * 4 + location.x, self.game_scale * 10 + location.y),

    }
}
