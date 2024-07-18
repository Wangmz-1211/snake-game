use std::fmt;

pub enum GameHardness {
    Easy,
    Normal, // default
    Hard,
}
impl fmt::Display for GameHardness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Easy => write!(f, "Easy"),
            Self::Normal => write!(f, "Normal"),
            Self::Hard => write!(f, "Hard"),
        }
    }
}
