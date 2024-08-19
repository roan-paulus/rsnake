use std::io;

use animate::{Animations, ChatboxAnimation};
use crossterm::cursor;
use crossterm::execute;

use crate::animate;
use crate::grid::Point;
use crate::object::banner::Banner;
use crate::object::food::Food;
use crate::object::snake::Snake;

const CONTINUE: bool = true;
const BREAK: bool = false;

fn colliding(a: Point, b: Point) -> bool {
    a.x == b.x && a.y == b.y
}

pub struct Game {
    pub points: u16,
    pub snake: Snake,
    pub food: Food,
    pub enemy: Option<Banner>,
    pub animations: Vec<Animations>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            points: 0,
            snake: Snake::new(5, 1),
            food: Food::new(),
            animations: Vec::new(),
            enemy: Some(Banner::new()),
        }
    }

    pub fn update(&mut self) -> crossterm::Result<bool> {
        // NOTE: This update also draws.
        if !self.snake.update()? {
            return Ok(BREAK);
        }

        if self.enemy.is_some() && !self.enemy.as_mut().unwrap().update()? {
            self.enemy = None;
        }

        let mut animation_remove_queue: Vec<usize> = Vec::new();
        self.animations
            .iter_mut()
            .enumerate()
            .for_each(|(i, animation)| {
                const STOPPED: bool = false;
                let playing = match animation {
                    Animations::Chatbox(animation) => animation.play(&self.snake),
                };
                if playing == STOPPED {
                    animation_remove_queue.push(i)
                }
            });

        animation_remove_queue.iter().for_each(|i| {
            self.animations.remove(*i);
        });

        let head = self.snake.get_head().get_point();

        if colliding(
            Point {
                x: self.food.x,
                y: self.food.y,
            },
            head,
        ) {
            self.animations
                .push(Animations::Chatbox(ChatboxAnimation::new(6)));
            self.points += 1;
            self.food.respawn();
            self.snake.grow();
        }

        Ok(CONTINUE)
    }

    pub fn draw(&self) {
        self.food.draw();
        if self.enemy.is_some() {
            self.enemy.as_ref().unwrap().draw();
        }
        self.draw_border();
        self.draw_points();
    }

    pub fn draw_border(&self) {
        let size = crossterm::terminal::size().unwrap();

        for x in 0..size.0 {
            execute!(io::stdout(), cursor::MoveTo(x, 0)).unwrap();
            print!("█");
        }

        for x in 0..size.0 {
            execute!(io::stdout(), cursor::MoveTo(x, size.1)).unwrap();
            print!("█");
        }

        for y in 0..size.1 {
            execute!(io::stdout(), cursor::MoveTo(0, y)).unwrap();
            print!("█");
        }

        for y in 0..size.1 {
            execute!(io::stdout(), cursor::MoveTo(size.0, y)).unwrap();
            print!("█");
        }
    }

    pub fn draw_points(&self) {
        execute!(io::stdout(), cursor::MoveTo(0, 0)).unwrap();
        print!("Points: {}", self.points);
    }
}
