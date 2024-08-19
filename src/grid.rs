#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u16,
    pub y: u16,
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
}
