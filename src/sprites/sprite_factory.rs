use crate::game_objects::coordinate::Coordinate;
use crate::sprites::enemies::BasicEnemy;
use crate::sprites::player::Player;
use crate::sprites::sprites::Sprite;

pub struct SpriteFactory {
    pub game_scale: i32,
    maximum_window_x_size: i32,
}

pub trait EnemyFactory {
    fn get_basic_enemy(&self, coordinate: Coordinate) -> BasicEnemy;
    fn get_basic_enemies(&self, coordinates: Vec<Coordinate>) -> Vec<BasicEnemy>;
}

impl EnemyFactory for SpriteFactory {
    fn get_basic_enemy(&self, coordinate: Coordinate) -> BasicEnemy {
        BasicEnemy::new(self.game_scale, coordinate, self.maximum_window_x_size)
    }

    fn get_basic_enemies(&self, coordinates:Vec<Coordinate>) -> Vec<BasicEnemy> {
        let mut enemies: Vec<BasicEnemy> = vec![];
        for coord in coordinates {
            enemies.push(BasicEnemy::new(self.game_scale, coord, self.maximum_window_x_size));
        }
        return enemies;
    }
}

pub trait PlayerFactory {
    fn get_player(&self, location:Coordinate) -> Player;
}

impl PlayerFactory for SpriteFactory {
    fn get_player(&self, location:Coordinate) -> Player {
        Player::new(self.game_scale, location, self.maximum_window_x_size)
    }
}

impl SpriteFactory {
    pub fn new(game_scale: i32, maximum_window_x_size: i32) -> Self {
        Self { 
            game_scale: game_scale, maximum_window_x_size = maximum_window_x_size
        }
    }
}
