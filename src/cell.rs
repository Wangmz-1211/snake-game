#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Cell {
    Food, // When snake get this, level up
    Body, // This is the snake body
    Head, // This is the snake head, only for display use.
    Blank,
}
