use std::cmp::Ordering;

use crossterm::event::{Event, KeyCode, KeyEventKind};

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
#[test]
fn get_next_up() {
    use crossterm::event::{KeyEvent, KeyEventState, KeyModifiers};

    let pos = Position { x: 3, y: 3 };
    let event = Event::Key(KeyEvent {
        code: KeyCode::Up,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    });
    let res = pos.get_next(event, 5, 5);
    assert_eq!(res, Ok(Position { x: 2, y: 3 }));
}
#[test]
fn get_next_down() {
    use crossterm::event::{KeyEvent, KeyEventState, KeyModifiers};

    let pos = Position { x: 3, y: 3 };
    let event = Event::Key(KeyEvent {
        code: KeyCode::Down,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    });
    let res = pos.get_next(event, 5, 5);
    assert_eq!(res, Ok(Position { x: 4, y: 3 }));
}
#[test]
fn get_next_left() {
    use crossterm::event::{KeyEvent, KeyEventState, KeyModifiers};

    let pos = Position { x: 3, y: 3 };
    let event = Event::Key(KeyEvent {
        code: KeyCode::Left,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    });
    let res = pos.get_next(event, 5, 5);
    assert_eq!(res, Ok(Position { x: 3, y: 2 }));
}

#[test]
fn get_next_right() {
    use crossterm::event::{KeyEvent, KeyEventState, KeyModifiers};

    let pos = Position { x: 3, y: 3 };
    let event = Event::Key(KeyEvent {
        code: KeyCode::Right,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    });
    let res = pos.get_next(event, 5, 5);
    assert_eq!(res, Ok(Position { x: 3, y: 4 }));
}

#[test]
fn get_next_up_fail() {
    use crossterm::event::{KeyEvent, KeyEventState, KeyModifiers};

    let pos = Position { x: 0, y: 0 };
    let event = Event::Key(KeyEvent {
        code: KeyCode::Up,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    });
    let res = pos.get_next(event, 0, 0);
    assert_eq!(res, Err(String::from("Hit Bound")));
}
#[test]
fn get_next_down_fail() {
    use crossterm::event::{KeyEvent, KeyEventState, KeyModifiers};

    let pos = Position { x: 0, y: 0 };
    let event = Event::Key(KeyEvent {
        code: KeyCode::Down,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    });
    let res = pos.get_next(event, 0, 0);
    assert_eq!(res, Err(String::from("Hit Bound")));
}
#[test]
fn get_next_left_fail() {
    use crossterm::event::{KeyEvent, KeyEventState, KeyModifiers};

    let pos = Position { x: 0, y: 0 };
    let event = Event::Key(KeyEvent {
        code: KeyCode::Left,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    });
    let res = pos.get_next(event, 0, 0);
    assert_eq!(res, Err(String::from("Hit Bound")));
}
#[test]
fn get_next_right_fail() {
    use crossterm::event::{KeyEvent, KeyEventState, KeyModifiers};

    let pos = Position { x: 0, y: 0 };
    let event = Event::Key(KeyEvent {
        code: KeyCode::Right,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    });
    let res = pos.get_next(event, 0, 0);
    assert_eq!(res, Err(String::from("Hit Bound")));
}
