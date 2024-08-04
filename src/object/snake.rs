use std::io;

use crossterm::event::{KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::{cursor, event::KeyModifiers};

use crate::grid::{Direction, Point};
use crate::{BREAK, CONTINUE};

use utils::SnakePoint;

pub struct Snake {
    // Body starts with the head
    pub body: Vec<SnakePoint>,
    pub speed: u16,
    pub direction: Direction,
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            body: vec![SnakePoint::new(x, y), SnakePoint::new(x - 1, y)],
            speed: 1,
            direction: Direction::Down,
        }
    }

    pub fn grow(&mut self) {
        self.body.push(SnakePoint::new(0, 0));
    }

    pub fn update(&self) -> crossterm::Result<bool> {
        if self.losing_condition() {
            return Ok(BREAK);
        }
        Ok(CONTINUE)
    }

    fn losing_condition(&self) -> bool {
        let mut body = self.body.iter();
        let head = body
            .next()
            .expect("Always initializes with a head.")
            .get_point();

        for part in body {
            let Point { x, y } = part.get_point();
            if head.x == x && head.y == y {
                return true;
            }
        }
        false
    }

    pub fn update_direction(&mut self, event: KeyEvent) -> bool {
        let KeyEvent { code, modifiers } = event;
        match (code, modifiers) {
            (KeyCode::Char('h'), KeyModifiers::NONE) => {
                const DESIRED_DIRECTION: Direction = Direction::Left;
                if self.direction.is_perpendicular(DESIRED_DIRECTION) {
                    self.direction = DESIRED_DIRECTION;
                }
            }
            (KeyCode::Char('j'), KeyModifiers::NONE) => {
                const DESIRED_DIRECTION: Direction = Direction::Down;
                if self.direction.is_perpendicular(DESIRED_DIRECTION) {
                    self.direction = DESIRED_DIRECTION;
                }
            }
            (KeyCode::Char('k'), KeyModifiers::NONE) => {
                const DESIRED_DIRECTION: Direction = Direction::Up;
                if self.direction.is_perpendicular(DESIRED_DIRECTION) {
                    self.direction = DESIRED_DIRECTION;
                }
            }
            (KeyCode::Char('l'), KeyModifiers::NONE) => {
                const DESIRED_DIRECTION: Direction = Direction::Right;
                if self.direction.is_perpendicular(DESIRED_DIRECTION) {
                    self.direction = DESIRED_DIRECTION;
                }
            }
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => return BREAK,
            _ => (),
        }
        CONTINUE
    }

    pub fn move_in(direction: Direction, body_part: &mut SnakePoint, speed: u16) {
        match direction {
            Direction::Left => body_part.move_left(speed),
            Direction::Down => body_part.move_down(speed),
            Direction::Up => body_part.move_up(speed),
            Direction::Right => body_part.move_right(speed),
        }
    }

    pub fn draw(body_part: &SnakePoint, shape: char) {
        let Point { x, y } = body_part.get_point();
        execute!(io::stdout(), cursor::MoveTo(x, y)).unwrap();
        print!("{shape}");
    }
}

mod utils {
    use super::*;

    pub struct SnakePoint {
        location: Point,
        direction_moved: Option<Direction>,
    }

    impl SnakePoint {
        pub fn new(x: u16, y: u16) -> Self {
            Self {
                location: Point { x, y },
                direction_moved: None,
            }
        }

        pub fn get_point(&self) -> Point {
            self.location
        }

        pub fn get_shape(&self) -> char {
            '■'
        }

        pub fn get_head_shape(&self) -> char {
            if let Some(direction) = self.direction_moved {
                match direction {
                    Direction::Left => '◄',
                    Direction::Down => '▼',
                    Direction::Up => '▲',
                    Direction::Right => '►',
                }
            } else {
                '?'
            }
        }

        pub fn goto(&mut self, point: Point) {
            self.location = point;
        }

        pub fn move_left(&mut self, speed: u16) {
            self.direction_moved = Some(Direction::Left);
            self.location.x -= speed;
        }

        pub fn move_down(&mut self, speed: u16) {
            self.direction_moved = Some(Direction::Down);
            self.location.y += speed;
        }

        pub fn move_up(&mut self, speed: u16) {
            self.direction_moved = Some(Direction::Up);
            self.location.y -= speed;
        }

        pub fn move_right(&mut self, speed: u16) {
            self.direction_moved = Some(Direction::Right);
            self.location.x += speed;
        }
    }
}
