use crate::sprites::sprite_factory::{ SpriteFactory, EnemyFactory } ;

struct GameController {
    score: i32,
    game_state: GameState::InProgress,
    sprite_factory: SpriteFactory,
    enemies: Vec<BasicEnemy>,
    player: Vec<Player>,
}

pub trait GameManagement [
    fn update_on_game_frame();
    fn render_objects();
    fn get_game_state();
}

impl GameManagement for GameController {
    fn update_on_game_frame(&self){
        self.score += 1;
        if self.score % 2 == 0 {

            for i in 0..self.enemies.len() {
                self.enemies[i].move_object(HorizontalDirection::Right);
            }
        }
    }

    fn render_objects(&self, graphics_library: &GlGraphics) {
        graphics_library.draw(args.viewport(), |c, gl|{
            clear(BLACK, gl);
            let transform = c.transform.trans(0.0,0.0).rot_deg(0.0);
            for enemy in &self.enemies {
                for pixel in enemy.get_screen_segments() {
                    let printable_segment = rectangle::square(pixel.x as f64, pixel.y as f64, 10.0);

                    rectangle(WHITE, printable_segment, transform, graphics_library);
                }
            }
        });
    }

    fn get_game_state(&self) -> GameState {
        if !self.enemies.iter().any() {
            GameState::Won
        }
        else if self.enemies.iter().any(|enemy| enemy.touching_bottom_screen_edge() {
            GameState::Lost
        }
        else {
            GameState::Ongoing
        }
    }

    fn get_score(&self) -> i32 {
        self.score
    }
}

impl GameController {
    pub fn new(
}
