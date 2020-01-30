pub use self::game::Game;
use std::io::{stdin, stdout};
use termion::raw::IntoRawMode;
mod game;

fn main() {
  let stdin = stdin();
  let stdout = stdout().into_raw_mode().unwrap();
  let mut game = Game {
    grid: vec![],
    stdout: stdout,
    stdin: stdin,
  };
  game.load_map("map.txt".to_string());
  game.run();
}
