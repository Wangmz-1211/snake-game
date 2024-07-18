use std::cmp::Ordering;

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn get_next(&self, direction: char, bound: usize) -> Result<Position, String> {
        match direction {
            'h' => match self.y {
                0 => Err(String::from("Hit Bound")),
                _ => Ok(Position {
                    x: self.x,
                    y: self.y - 1,
                }),
            },
            'j' => match self.x.cmp(&bound) {
                Ordering::Equal => Err(String::from("Hit Bound")),
                _ => Ok(Position {
                    x: self.x + 1,
                    y: self.y,
                }),
            },

            'k' => match self.x {
                0 => Err(String::from("Hit Bound")),
                _ => Ok(Position {
                    x: self.x - 1,
                    y: self.y,
                }),
            },
            'l' => match self.y.cmp(&bound) {
                Ordering::Equal => Err(String::from("Hit Bound")),
                _ => Ok(Position {
                    x: self.x,
                    y: self.y + 1,
                }),
            },
            _ => Err(String::from("Wrong direction.")),
        }
    }
}
