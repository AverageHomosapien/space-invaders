use crate::game_objects::coordinate::Coordinate;
use crate::sprites::enemies::BasicEnemy;
use crate::sprites::player::Player;
use crate::sprites::sprites::Sprite;

pub struct SpriteFactory {
    pub game_scale: i32
}

pub trait EnemyFactory {
    fn get_basic_enemy(&self, coordinate: Coordinate, maximum_window_x_size:u32) -> BasicEnemy;
    fn get_basic_enemies(&self, coordinates: Vec<Coordinate>, maximum_window_x_size:u32) -> Vec<BasicEnemy>;
}

impl EnemyFactory for SpriteFactory {
    fn get_basic_enemy(&self, coordinate: Coordinate, maximum_window_x_size:u32) -> BasicEnemy {
        BasicEnemy::new(self.game_scale,  coordinate, maximum_window_x_size)
    }

    fn get_basic_enemies(&self, coordinates:Vec<Coordinate>, maximum_window_x_size:u32) -> Vec<BasicEnemy> {
        let mut enemies: Vec<BasicEnemy> = vec![];
        for coord in coordinates {
            enemies.push(BasicEnemy::new(self.game_scale, coord, maximum_window_x_size));
        }
        return enemies;
    }
}

pub trait PlayerFactory {
    fn get_player(&self, location:Coordinate) -> Player;
}

impl PlayerFactory for SpriteFactory {
    fn get_player(&self, location:Coordinate) -> Player {
        Player::new(self.game_scale,  location)
    }
}
