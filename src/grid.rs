use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    fn direction(&self, other: &Self) -> Direction {
        // Does not take points into account that are diagonally from eachother.
        match self.x.cmp(&other.x) {
            Ordering::Less => Direction::Right,
            Ordering::Greater => Direction::Left,
            Ordering::Equal => match self.y.cmp(&other.y) {
                Ordering::Less => Direction::Down,
                Ordering::Greater => Direction::Up,
                Ordering::Equal => panic!("Point is exactly on another point."),
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

impl Direction {
    pub fn is_perpendicular(&self, desired_direction: Self) -> bool {
        if *self == desired_direction {
            return false;
        }

        match self {
            Self::Left => desired_direction != Self::Right,
            Self::Down => desired_direction != Self::Up,
            Self::Up => desired_direction != Self::Down,
            Self::Right => desired_direction != Self::Left,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }
}
