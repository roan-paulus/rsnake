use crossterm::{cursor, execute};
use rand::{self, Rng};
use std::io;

use super::snake::Snake;

pub struct Food {
    pub x: u16,
    pub y: u16,
}

impl Food {
    pub fn eaten_by(&mut self, snake: &Snake) -> crossterm::Result<bool> {
        let head = snake.body.first().unwrap().get_point();

        if head.x == self.x && head.y == self.y {
            return Ok(true);
        }

        Ok(false)
    }

    pub fn draw(&self) {
        execute!(io::stdout(), cursor::MoveTo(self.x, self.y)).unwrap();
        print!("■");
    }

    pub fn new() -> Self {
        Self { x: 15, y: 15 }
    }

    pub fn respawn(&mut self) {
        let (max_cols, max_rows) = match crossterm::terminal::size() {
            Ok(size) => size,
            Err(_) => panic!("Fuck!"),
        };

        self.x = rand::thread_rng().gen_range(1..max_cols);
        self.y = rand::thread_rng().gen_range(1..max_rows);
    }
}
