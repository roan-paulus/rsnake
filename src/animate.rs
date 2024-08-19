use crate::helpers::qprint;
use crate::helpers::qprintln;
use crate::object::snake::Snake;
use crate::Point;

pub type Playing = bool;

pub enum Animations {
    Chatbox(ChatboxAnimation),
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
