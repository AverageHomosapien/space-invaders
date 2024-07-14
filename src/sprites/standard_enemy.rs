pub struct BasicEnemy {
    u32: game_scale,
}

impl BasicEnemy{
    fn get_screen_segments(&mut self,  Coordinate: location) -> Vec<Coordinate> {
        vec![Coordinate{x:0 + location.x, y:self.game_scale * 2 + location.y}, Coordinate {x:0 + location.x, y:self.game_scale * 8 + location.y},
             Coordinate{x:self.game_scale * 1 + location.x, y:self.game_scale * 3 + location.y}, Coordinate{x: self.game_scale * 1 + location.x, y:self.game_scale * 7 + location.y}]
    }
}
