use crossterm::{
    execute,
    style::{Color, ResetColor, SetForegroundColor},
};

use crate::{
    grid::{Direction, Point},
    helpers::qprint,
};

pub struct Bullet {
    pub location: Point,
    direction: Direction,
}

type OutOfBounds = bool;

impl Bullet {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            location: Point { x, y },
            direction: Direction::Down,
        }
    }

    pub fn update(&mut self) -> crossterm::Result<OutOfBounds> {
        use Direction as D;
        match self.direction {
            D::Up => self.location.y -= 1,
            D::Down => self.location.y += 1,
            D::Left => self.location.x -= 1,
            D::Right => self.location.x += 1,
        }

        let (_, rows) = match crossterm::terminal::size() {
            Ok((c, r)) => (c, r),
            Err(_) => return Ok(true),
        };

        if self.location.y >= rows {
            return Ok(true);
        }
        Ok(false)
    }

    pub fn draw(&self) {
        execute!(std::io::stdout(), SetForegroundColor(Color::DarkGreen)).unwrap();
        qprint('█', self.location);
        execute!(std::io::stdout(), ResetColor).unwrap();
    }
}
