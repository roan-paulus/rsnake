use crossterm::{cursor, execute};
use std::io;

use crate::{Game, CONTINUE};

use super::snake::Snake;

pub struct Food {
    x: u16,
    y: u16,
}

impl Food {
    pub fn update(&mut self, snake: &mut Snake, game: &mut Game) -> crossterm::Result<bool> {
        let head = snake.body.first().unwrap().get_point();

        if head.x == self.x && head.y == self.y {
            game.points += 1;
            self.respawn();
            snake.grow();
        }

        Ok(CONTINUE)
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
