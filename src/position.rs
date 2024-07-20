use std::cmp::Ordering;

use crossterm::event::{Event, KeyCode, KeyEventKind};

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn get_next(
        &self,
        event: Event,
        x_bound: usize,
        y_bound: usize,
    ) -> Result<Position, String> {
        match event {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                KeyCode::Left => match self.y {
                    0 => Err(String::from("Hit Bound")),
                    _ => Ok(Position {
                        x: self.x,
                        y: self.y - 1,
                    }),
                },
                KeyCode::Up => match self.x {
                    0 => Err(String::from("Hit Bound")),
                    _ => Ok(Position {
                        x: self.x - 1,
                        y: self.y,
                    }),
                },
                KeyCode::Down => match self.x.cmp(&x_bound) {
                    Ordering::Equal => Err(String::from("Hit Bound")),
                    _ => Ok(Position {
                        x: self.x + 1,
                        y: self.y,
                    }),
                },
                KeyCode::Right => match self.y.cmp(&y_bound) {
                    Ordering::Equal => Err(String::from("Hit Bound")),
                    _ => Ok(Position {
                        x: self.x,
                        y: self.y + 1,
                    }),
                },
                _ => Err(String::from("Unknown key code")),
            },
            _ => Err(String::from("Wrong direction.")),
        }
    }
}
