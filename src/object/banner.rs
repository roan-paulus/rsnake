use crate::{grid::Point, helpers::qprintln};

pub struct Banner {
    health: u32,
    location: Point,
    shape: &'static str,
}

type Alive = bool;
const LIVING: bool = true;
const DIED: bool = false;

impl Banner {
    pub fn new() -> Self {
        Self {
            health: 1,
            location: Point { x: 2, y: 1 },
            shape: "__S_N_A_K_E__",
        }
    }

    pub fn update(&mut self) -> crossterm::Result<Alive> {
        let (columns, _) = crossterm::terminal::size()?;

        self.health = self.health.saturating_sub(0);
        if self.health == 0 {
            return Ok(DIED);
        }

        if self.location.x < columns - self.shape.len() as u16 {
            self.location.x += 1;
        } else {
            return Ok(DIED);
        }
        Ok(LIVING)
    }

    pub fn draw(&self) {
        qprintln(self.shape, self.location);
    }
}
