pub use self::game::Game;
mod game;

fn main() {
  let mut game = Game { grid: vec![] };
  game.load_map("map.txt".to_string());
  game.run();
}
