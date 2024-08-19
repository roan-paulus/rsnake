use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

use crossterm::cursor;
use crossterm::event::Event;
use crossterm::{event, execute, terminal};
use grid::Point;

use crate::game::Game;

mod animate;
mod game;
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
