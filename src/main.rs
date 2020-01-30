pub use self::game::Game;
pub use self::player::Player;
use std::io::stdout;
use termion::raw::IntoRawMode;

mod game;
mod player;

fn main() {
  let player = Player { x: 2, y: 3 };
  let stdout = stdout().into_raw_mode().unwrap();
  let mut game = Game {
    grid: vec![],
    stdout: stdout,
    player: player,
  };
  game.load_map("map.txt".to_string());
  game.run();
}
