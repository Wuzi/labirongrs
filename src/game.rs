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
                termion::cursor::Goto(1, i as u16 + 1),
                termion::clear::CurrentLine,
                row,
            )
            .unwrap();
        }

        // draw player
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(self.player.x + 1, self.player.y + 1),
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

            let mut x: u16 = self.player.x;
            let mut y: u16 = self.player.y;

            match c.unwrap() {
                Key::Esc => break,
                Key::Left => x = self.player.x - 1,
                Key::Right => x = self.player.x + 1,
                Key::Up => y = self.player.y - 1,
                Key::Down => y = self.player.y + 1,
                _ => (),
            }

            if self.grid[y as usize].as_bytes()[x as usize] as char != '#' {
                self.player.x = x;
                self.player.y = y;
            }

            self.print_screen();
            self.stdout.flush().unwrap();
        }

        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
    }
}
