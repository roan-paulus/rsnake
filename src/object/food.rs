use crossterm::{cursor, execute};
use std::io;

use crate::{Game, CONTINUE};

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
        self.x += 1;
        self.y += 1
    }
}
