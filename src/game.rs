use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Game {
  pub grid: Vec<String>,
}

impl Game {
  pub fn print_screen(&self) {
    for row in &self.grid {
      println!("{}", row);
    }
  }

  fn clear_screen(&self) {
    print!("\x1B[2J");
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

  pub fn run(self) {
    if self.grid.len() < 1 {
      println!("You need to load a map first!");
      return;
    }
    self.clear_screen();
    self.print_screen();
  }
}
