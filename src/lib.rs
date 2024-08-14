use std::{
    io::{self, Read, Write},
    thread::sleep,
    time::{Duration, Instant},
};

use animate::{Animations, ChatboxAnimation};
use crossterm::cursor;
use crossterm::event::Event;
use crossterm::{event, execute, terminal};
use grid::Point;

use crate::object::food::Food;
use crate::object::snake::Snake;

mod animate;
mod grid;
mod helpers;
mod object;

const CONTINUE: bool = true;
const BREAK: bool = false;

pub fn run() -> crossterm::Result<String> {
    let _clean_up_code = Term::init();
    let mut game = Game::new();

    loop {
        if event::poll(Duration::from_secs_f32(0.001))? {
            match event::read() {
                Ok(Event::Key(key_event)) => {
                    if game.snake.update_direction(key_event) == BREAK {
                        return Ok(String::from("\nGamed stopped with [q]."));
                    };
                }
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }

        execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;

        if !game.update()? {
            return Ok(format!("\nGame over.\nFinal score: {}", game.points));
        }
        game.draw();

        io::stdout().flush().unwrap();

        sleep(Duration::from_secs_f32(0.1));
    }
}

struct Game {
    points: u16,
    snake: Snake,
    food: Food,
    animations: Vec<Animations>,
}

impl Game {
    fn new() -> Self {
        Self {
            points: 0,
            snake: Snake::new(5, 1),
            food: Food::new(),
            animations: Vec::new(),
        }
    }

    fn update(&mut self) -> crossterm::Result<bool> {
        // NOTE: This update also draws.
        if !self.snake.update()? {
            return Ok(BREAK);
        }

        let mut animation_remove_queue: Vec<usize> = Vec::new();
        self.animations
            .iter_mut()
            .enumerate()
            .for_each(|(i, animation)| {
                const STOPPED: bool = false;
                let playing = match animation {
                    Animations::Chatbox(animation) => animation.play(&self.snake),
                    Animations::Random(animation) => animation.play(),
                };
                if playing == STOPPED {
                    animation_remove_queue.push(i)
                }
            });

        animation_remove_queue.iter().for_each(|i| {
            self.animations.remove(*i);
        });

        if self.food.eaten_by(&self.snake)? {
            self.animations
                .push(Animations::Chatbox(ChatboxAnimation::new(6)));
            self.points += 1;
            self.food.respawn();
            self.snake.grow();
        }

        Ok(CONTINUE)
    }

    fn draw(&self) {
        self.food.draw();
        self.draw_border();
        self.draw_points();
    }

    fn draw_border(&self) {
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
