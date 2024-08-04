use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::cursor;
use crossterm::event::Event;
use crossterm::{event, execute, terminal};

use crate::object::food::Food;
use crate::object::snake::Snake;

mod grid;
mod object;

const CONTINUE: bool = true;
const BREAK: bool = false;

pub fn run() -> crossterm::Result<()> {
    let _clean_up_code = Term::init();

    let mut game = Game { points: 0 };

    let mut snake = Snake::new(5, 1);
    let mut food = Food::new();

    let mut keep_looping = true;
    while keep_looping {
        if event::poll(Duration::from_millis(200))? {
            match event::read() {
                Ok(Event::Key(key_event)) => {
                    if snake.update_direction(key_event) == BREAK {
                        keep_looping = false;
                    };
                }
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }

        execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;

        keep_looping = snake.update()?;
        if !keep_looping {
            println!("You lose. Score: {}", game.points);
            break;
        }

        let mut body = snake.body.iter_mut();
        let head = body.next().unwrap();

        let mut previous_part = head.get_point();

        Snake::move_in(snake.direction, head, snake.speed);
        Snake::draw(head, head.get_head_shape());

        for body_part in body {
            let p = body_part.get_point();
            body_part.goto(previous_part);
            previous_part = p;
            Snake::draw(body_part, body_part.get_shape());
        }
        food.update(&mut snake, &mut game)?;
        food.draw();
        game.draw_points();
        io::stdout().flush().unwrap();
    }

    Ok(())
}

struct Game {
    points: u16,
}

impl Game {
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
