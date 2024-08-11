use std::io;

use crossterm::{cursor, execute};

use crate::Point;

pub fn qprint(c: char, point: Point) {
    execute!(io::stdout(), cursor::MoveTo(point.x, point.y)).unwrap();
    print!("{c}");
}

pub fn qprintln(s: &str, point: Point) {
    execute!(io::stdout(), cursor::MoveTo(point.x, point.y)).unwrap();
    print!("{s}");
}
