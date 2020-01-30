extern crate termion;
use crate::player;

use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Write};
use termion::event::Key;
use termion::input::TermRead;

pub struct Game {
  pub grid: Vec<String>,
  pub stdout: termion::raw::RawTerminal<std::io::Stdout>,
  pub player: player::Player,
}

impl Game {
  pub fn print_screen(&mut self) {
    // draw grid
    for (i, row) in self.grid.iter().enumerate() {
      write!(
        self.stdout,
        "{}{}{}",
        termion::cursor::Goto(1, i as u16 + 2),
        termion::clear::CurrentLine,
        row,
      )
      .unwrap();
    }

    // draw player
    write!(
      self.stdout,
      "{}{}",
      termion::cursor::Goto(self.player.x, self.player.y),
      "P",
    )
    .unwrap();

    self.stdout.flush().unwrap();
  }

  fn clear_screen(&mut self) {
    write!(
      self.stdout,
      "{}{}{}",
      termion::clear::All,
      termion::cursor::Goto(1, 1),
      termion::cursor::Hide,
    )
    .unwrap();
    self.stdout.flush().unwrap();
  }

  pub fn load_map(&mut self, filename: String) {
    let file = File::open(filename).expect("Could not read file");
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
      let line = match line {
        Ok(line) => line,
        Err(e) => e.to_string(),
      };
      self.grid.push(line);
    }
  }

  pub fn run(&mut self) {
    if self.grid.len() < 1 {
      println!("You need to load a map first!");
      return;
    }

    self.clear_screen();
    self.print_screen();

    let stdin = stdin();
    for c in stdin.keys() {
      self.clear_screen();

      match c.unwrap() {
        Key::Esc => break,
        Key::Left => self.player.x -= 1,
        Key::Right => self.player.x += 1,
        Key::Up => self.player.y -= 1,
        Key::Down => self.player.y += 1,
        _ => (),
      }

      self.print_screen();
      self.stdout.flush().unwrap();
    }

    write!(self.stdout, "{}", termion::cursor::Show).unwrap();
  }
}
