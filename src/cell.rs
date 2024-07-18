#[derive(Clone, Debug)]
pub enum Cell {
    Wall, // When the snake get this, dead.
    Food, // When snake get this, level up
    Body, // This is the snake body
    Head, // This is the snake head, only for display use.
    Blank,
}
