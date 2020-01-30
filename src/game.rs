extern crate termion;

use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Write};
use termion::event::Key;
use termion::input::TermRead;

pub struct Game {
  pub grid: Vec<String>,
  pub stdout: termion::raw::RawTerminal<std::io::Stdout>,
}

impl Game {
  pub fn print_screen(&mut self) {
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
      self.print_screen();

      write!(
        self.stdout,
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::CurrentLine,
      )
      .unwrap();

      match c.unwrap() {
        Key::Esc => break,
        Key::Left => println!("<left>"),
        Key::Right => println!("<right>"),
        Key::Up => println!("<up>"),
        Key::Down => println!("<down>"),
        _ => (),
      }

      self.stdout.flush().unwrap();
    }

    write!(self.stdout, "{}", termion::cursor::Show).unwrap();
  }
}
