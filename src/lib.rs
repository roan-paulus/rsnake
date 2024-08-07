use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::cursor;
use crossterm::event::Event;
use crossterm::{event, execute, terminal};
use grid::Point;

use crate::animate::Animation;
use crate::object::food::Food;
use crate::object::snake::Snake;

mod animate;
mod grid;
mod object;

const CONTINUE: bool = true;
const BREAK: bool = false;

pub fn run() -> crossterm::Result<()> {
    let _clean_up_code = Term::init();
    let mut game = Game::new();

    loop {
        if event::poll(Duration::from_millis(200))? {
            match event::read() {
                Ok(Event::Key(key_event)) => {
                    if game.snake.update_direction(key_event) == BREAK {
                        return Ok(());
                    };
                }
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }

        execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;

        if !game.update()? {
            return Ok(());
        }
        game.draw();

        io::stdout().flush().unwrap();
    }
}

struct Game {
    points: u16,
    snake: Snake,
    food: Food,
    animation: Option<Animation>,
}

impl Game {
    fn new() -> Self {
        Self {
            points: 0,
            snake: Snake::new(5, 1),
            food: Food::new(),
            animation: None,
        }
    }

    fn update(&mut self) -> crossterm::Result<bool> {
        // NOTE: This update also draws.
        if !self.snake.update()? {
            return Ok(BREAK);
        }

        if let Some(animation) = self.animation.as_mut() {
            const STOPPED: bool = false;
            if animation.play() == STOPPED {
                self.animation = None;
            }
        }

        if self.food.eaten_by(&self.snake)? {
            let snake_location = self.snake.body.first().unwrap().get_point();

            self.animation = Some(Animation::from(snake_location.x, snake_location.y));
            self.points += 1;
            self.food.respawn();
            self.snake.grow();
        }

        Ok(CONTINUE)
    }

    fn draw(&self) {
        self.food.draw();
        self.draw_points();
    }

    fn draw_points(&self) {
        execute!(io::stdout(), cursor::MoveTo(0, 0)).unwrap();
        print!("Points: {}", self.points);
    }
}

struct Term;

impl Term {
    fn init() -> Self {
        execute!(io::stdout(), cursor::Hide).expect("Hiding cursor failed");
        terminal::enable_raw_mode().expect("Cannot enable raw mode");
        Self
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Cannot disable rawmode");
        execute!(io::stdout(), cursor::Show).expect("Showing cursor failed");
    }
}
