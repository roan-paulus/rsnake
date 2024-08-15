use crate::{grid::Point, helpers::qprintln};

use super::bullet::Bullet;

pub struct Enemy {
    health: u32,
    location: Point,
    shape: &'static str,
    bullets_shot: Vec<Bullet>,
}

type Alive = bool;
const LIVING: bool = true;
const DIED: bool = false;

impl Enemy {
    pub fn new() -> Self {
        Self {
            health: 1,
            location: Point { x: 2, y: 1 },
            shape: "(\\˶°ㅁ°)/",
            bullets_shot: Vec::new(),
        }
    }

    pub fn update(&mut self) -> crossterm::Result<Alive> {
        let (columns, _) = crossterm::terminal::size()?;

        if self.bullets_shot.is_empty() {
            self.bullets_shot
                .push(Bullet::new(self.location.x, self.location.y));
        }

        self.bullets_shot.iter_mut().for_each(|bullet| {
            bullet.update();
            bullet.draw();
        });

        self.health = self.health.saturating_sub(0);
        if self.health == 0 {
            return Ok(DIED);
        }

        if self.location.x < columns - self.shape.len() as u16 {
            self.location.x += 1;
        }
        Ok(LIVING)
    }

    pub fn draw(&self) {
        qprintln(self.shape, self.location);
    }

    pub fn get_bullets(&self) -> &Vec<Bullet> {
        &self.bullets_shot
    }
}
