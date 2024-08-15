use crate::{
    grid::{Direction, Point},
    helpers::qprint,
};

pub struct Bullet {
    location: Point,
    direction: Direction,
}

impl Bullet {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            location: Point { x, y },
            direction: Direction::Down,
        }
    }

    pub fn update(&mut self) {
        use Direction as D;
        match self.direction {
            D::Up => self.location.y -= 1,
            D::Down => self.location.y += 1,
            D::Left => self.location.x -= 1,
            D::Right => self.location.x += 1,
        }
    }

    pub fn draw(&self) {
        qprint('█', self.location)
    }
}
