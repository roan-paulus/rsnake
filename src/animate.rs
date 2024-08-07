use std::io;

use crossterm::cursor;
use crossterm::execute;

use crate::Point;

const SYMBOL_SIZE: usize = 8;

pub struct Animation {
    point: Point,
    counter: usize,
    symbols: [char; SYMBOL_SIZE],
    state: State,
}

type Playing = bool;

enum State {
    MovingOut,
    Downward,
}

impl Animation {
    pub fn from(x: u16, y: u16) -> Self {
        Self {
            point: Point { x, y },
            counter: 0,
            symbols: ['N', 'N', 'I', 'I', 'C', 'C', 'E', 'E'],
            state: State::MovingOut,
        }
    }

    pub fn play(&mut self) -> Playing {
        const CONTINUE: bool = true;
        const STOP: bool = true;

        if self.counter >= SYMBOL_SIZE * 2 {
            return STOP;
        }

        execute!(io::stdout(), cursor::MoveTo(self.point.x, self.point.y)).unwrap();
        print!("{}", self.symbols[self.counter % SYMBOL_SIZE]);

        use State as s;
        match self.state {
            s::MovingOut => {
                if self.counter > 1 {
                    self.state = s::Downward;
                }
                self.point.x += 1;
            }
            s::Downward => self.point.y += 1,
        }
        self.counter += 1;

        CONTINUE
    }
}
