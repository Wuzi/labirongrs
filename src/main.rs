pub use self::game::Game;
use std::io::{stdout};
use termion::raw::IntoRawMode;
mod game;

fn main() {
  let stdout = stdout().into_raw_mode().unwrap();
  let mut game = Game {
    grid: vec![],
    stdout: stdout,
  };
  game.load_map("map.txt".to_string());
  game.run();
}
