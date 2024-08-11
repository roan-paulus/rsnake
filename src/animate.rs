use std::io;

use crossterm::cursor;
use crossterm::execute;

use crate::helpers::qprint;
use crate::helpers::qprintln;
use crate::object::snake::Snake;
use crate::Point;

const SYMBOL_SIZE: usize = 8;

pub type Playing = bool;

trait _Animation {
    fn play(&mut self) -> Playing;
}

pub enum Animations {
    Chatbox(ChatboxAnimation),
    Random(RandomAnimation),
}

pub struct ChatboxAnimation {
    expiry_counter: u8,
}

impl ChatboxAnimation {
    pub fn new(expiry_counter: u8) -> Self {
        Self { expiry_counter }
    }
}

impl ChatboxAnimation {
    pub fn play(&mut self, snake: &Snake) -> Playing {
        const BALLOON_STRING: char = '/';

        if self.expiry_counter == 0 {
            return false;
        }

        let snake_head = snake.body.first().unwrap().get_point();
        let end = 3;

        // Prevent overflow by not printing
        if snake_head.y < end {
            return true;
        }

        (1..end).for_each(|n| {
            qprint(
                BALLOON_STRING,
                Point {
                    x: snake_head.x + n,
                    y: snake_head.y - n,
                },
            )
        });
        qprintln(
            "Yum!",
            Point {
                x: snake_head.x + end,
                y: snake_head.y - end,
            },
        );
        self.expiry_counter -= 1;
        true
    }
}

pub struct RandomAnimation {
    point: Point,
    counter: usize,
    symbols: [char; SYMBOL_SIZE],
    state: State,
}

enum State {
    MovingOut,
    Downward,
}

impl RandomAnimation {
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
