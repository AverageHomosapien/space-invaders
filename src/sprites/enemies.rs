use crate::game_objects::coordinate::Coordinate;

pub trait Enemy {
    fn get_screen_segments(&self, location: Coordinate) -> Vec<Coordinate>;
}

pub struct BasicEnemy {
    pub game_scale: i32,
}

impl Enemy for BasicEnemy{
    fn get_screen_segments(&self,  location: Coordinate) -> Vec<Coordinate> {
        vec![Coordinate::new(0 + location.x, self.game_scale * 2 + location.y), Coordinate::new(0 + location.x, self.game_scale * 8 + location.y),
             Coordinate::new(self.game_scale * 1 + location.x, self.game_scale * 3 + location.y), Coordinate::new(self.game_scale * 1 + location.x, self.game_scale * 7 + location.y),
             Coordinate::new(self.game_scale * 2 + location.x, self.game_scale * 2 + location.y),Coordinate::new(self.game_scale * 2 + location.x, self.game_scale * 3 + location.y), 
                Coordinate::new(self.game_scale * 2 + location.x, self.game_scale * 4 + location.y), Coordinate::new(self.game_scale * 2 + location.x, self.game_scale * 5 + location.y),
                Coordinate::new(self.game_scale * 2 + location.x, self.game_scale * 6 + location.y), Coordinate::new(self.game_scale * 2 + location.x, self.game_scale * 7 + location.y),
                Coordinate::new(self.game_scale * 2 + location.x, self.game_scale * 8 + location.y),
            Coordinate::new(self.game_scale * 3 + location.x, self.game_scale * 1 + location.y),Coordinate::new(self.game_scale * 3 + location.x, self.game_scale * 2 + location.y), 
                Coordinate::new(self.game_scale * 3 + location.x, self.game_scale * 4 + location.y), Coordinate::new(self.game_scale * 3 + location.x, self.game_scale * 5 + location.y),
                Coordinate::new(self.game_scale * 3 + location.x, self.game_scale * 6 + location.y), Coordinate::new(self.game_scale * 3 + location.x, self.game_scale * 8 + location.y),
                Coordinate::new(self.game_scale * 3 + location.x, self.game_scale * 9 + location.y),
            Coordinate::new(self.game_scale * 4 + location.x, self.game_scale * 0 + location.y), Coordinate::new(self.game_scale * 4 + location.x, self.game_scale * 1 + location.y),
                Coordinate::new(self.game_scale * 4 + location.x, self.game_scale * 2 + location.y), Coordinate::new(self.game_scale * 4 + location.x, self.game_scale * 3 + location.y),
                Coordinate::new(self.game_scale * 4 + location.x, self.game_scale * 4 + location.y), Coordinate::new(self.game_scale * 4 + location.x, self.game_scale * 5 + location.y),
                Coordinate::new(self.game_scale * 4 + location.x, self.game_scale * 6 + location.y), 
                Coordinate::new(self.game_scale * 4 + location.x, self.game_scale * 7 + location.y), Coordinate::new(self.game_scale * 4 + location.x, self.game_scale * 8 + location.y),
                Coordinate::new(self.game_scale * 4 + location.x, self.game_scale * 9 + location.y), Coordinate::new(self.game_scale * 4 + location.x, self.game_scale * 10 + location.y),
            Coordinate::new(self.game_scale * 5 + location.x, self.game_scale * 0 + location.y),
                Coordinate::new(self.game_scale * 5 + location.x, self.game_scale * 2 + location.y), Coordinate::new(self.game_scale * 5 + location.x, self.game_scale * 3 + location.y),
                Coordinate::new(self.game_scale * 5 + location.x, self.game_scale * 4 + location.y), Coordinate::new(self.game_scale * 5 + location.x, self.game_scale * 5 + location.y),
                Coordinate::new(self.game_scale * 5 + location.x, self.game_scale * 6 + location.y), 
                Coordinate::new(self.game_scale * 5 + location.x, self.game_scale * 7 + location.y), Coordinate::new(self.game_scale * 5 + location.x, self.game_scale * 8 + location.y),
                Coordinate::new(self.game_scale * 5 + location.x, self.game_scale * 10 + location.y),
            Coordinate::new(self.game_scale * 6 + location.x, self.game_scale * 0 + location.y), Coordinate::new(self.game_scale * 6 + location.x, self.game_scale * 2 + location.y), 
                Coordinate::new(self.game_scale * 6 + location.x, self.game_scale * 8 + location.y), Coordinate::new(self.game_scale * 6 + location.x, self.game_scale * 10 + location.y),
            Coordinate::new(self.game_scale * 7 + location.x, self.game_scale * 2 + location.y), Coordinate::new(self.game_scale * 7 + location.x, self.game_scale * 3 + location.y),
                Coordinate::new(self.game_scale * 7 + location.x, self.game_scale * 7 + location.y), Coordinate::new(self.game_scale * 7 + location.x, self.game_scale * 8 + location.y)]
    }
}
