use std::{
    collections::VecDeque,
    fmt::{self},
};

#[derive(Clone, Debug)]
enum Cell {
    Wall, // When the snake get this, dead.
    Food, // When snake get this, level up
    Body, // This is the snake body
    Head, // This is the snake head, only for display use.
    Blank,
}

enum GameStatus {
    Initialize, // Initializing
    Running,    // Game is running
    Finished,   // Once finished, never run again
}

enum GameHardness {
    Easy,
    Normal, // default
    Hard,
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}
impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0 }
    }
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

struct Game {
    map: Vec<Vec<Cell>>,
    // Runtime Parameters
    status: GameStatus,
    timestamp: u32, // The process of the Game
    score: u32,     // The length of snake (l) is l = score + 5;
    length: u32,

    // Snake info
    snake: VecDeque<Position>,
    ate: bool,
    dead: bool,

    // Configuration
    hardness: GameHardness,
    map_size: usize,
}

impl Game {
    /**
    Initialize All prerequisit for a game.
    */
    fn new(map_size: usize) -> Game {
        // Initialize map
        // The minimum map_size is 5
        let mut map_size = map_size;
        if map_size < 5 {
            map_size = 5;
        }
        let mut map = vec![vec![Cell::Blank; map_size]; map_size];

        // Initialize snake
        // The init length of snake
        let mid = (map_size + 1) / 2;
        let length = 5.min(mid as u32);
        let mut snake = VecDeque::new();
        for i in 0..length {
            let y = mid - i as usize;
            // head is always at snake.front()
            snake.push_back(Position { x: mid, y });
            map[mid][y] = Cell::Body;
        }

        Game {
            map,

            timestamp: 0,
            score: 0,
            length,
            status: GameStatus::Initialize,

            snake,
            ate: false,
            dead: false,

            hardness: GameHardness::Normal,
            map_size,
        }
    }
    /**
    Run the game.
    */
    fn run(&mut self) {
        self.status = GameStatus::Running;
        let size = self.map_size as u32;
        let stdin = std::io::stdin();
        loop {
            if self.length == size * size {
                self.win();
            }

            self.timestamp = self.timestamp + 1;

            self.display_map();
            // mock input
            println!("Input the next direction (hjkl):");
            let mut buf = String::new();
            stdin.read_line(&mut buf).expect("Failed reading input.");
            let direction: Vec<char> = buf.trim().chars().collect();
            let mut next_pos: Position = Position::new();
            if let Some(curr_head) = self.snake.front() {
                next_pos = match direction[0] {
                    'h' => Position {
                        x: curr_head.x,
                        y: curr_head.y - 1,
                    },
                    'j' => Position {
                        x: curr_head.x + 1,
                        y: curr_head.y,
                    },
                    'k' => Position {
                        x: curr_head.x - 1,
                        y: curr_head.y,
                    },
                    'l' => Position {
                        x: curr_head.x,
                        y: curr_head.y + 1,
                    },
                    _ => Position::new(),
                };
            }
            self.move_snake(next_pos);
        }
    }

    fn win(&mut self) {
        self.status = GameStatus::Finished;
        println!(
            "\tCongratulations!\n\nYou won the game.\n map size: {}\n hardness: {}",
            self.map_size, self.hardness
        );
    }

    fn display_map(&mut self) {
        let head_pos = match self.snake.front() {
            Some(pos) => pos,
            None => &Position::new(),
        };
        self.map[head_pos.x][head_pos.y] = Cell::Head;
        for cells in &self.map {
            let l = cells.len();
            let mut chars: Vec<char> = vec![' '; l];
            for i in 0..l {
                chars[i] = match cells[i] {
                    Cell::Blank => '🐾',
                    Cell::Wall => '🧱',
                    Cell::Food => '🍎',
                    Cell::Body => '🐍',
                    Cell::Head => '👦',
                }
            }
            let chars: String = chars.iter().collect();
            println!("{}", chars);
        }
        self.map[head_pos.x][head_pos.y] = Cell::Body;
    }

    /**
    Move the snake head to the next block,
    Also handle the `ate` buff.
    */
    fn move_snake(&mut self, p: Position) {
        // check position valid
        if p.x >= self.map_size || p.y >= self.map_size {
            println!("Hit Wall! Game Over.");
            return;
        }
        if let Some(curr_head) = self.snake.front() {
            // The snake can only move 1 cell through 4 direction.
            let diff = curr_head.x.abs_diff(p.x) + curr_head.y.abs_diff(p.y);
            if diff != 1 {
                panic!(
                    "Move Position error\ncurr head; {:?}\ntarget pos: {:?}",
                    curr_head, p
                );
            }
            // Check the content of the target cell. If it's wall or something,
            // add flags to the game object.
            let content = &self.map[p.x][p.y];
            match content {
                Cell::Wall => {
                    self.dead = true;
                    panic!("Game Over");
                }
                Cell::Body => {
                    self.dead = true;
                    panic!("Game Over");
                }
                Cell::Food => {
                    self.score += 1;
                    self.length += 1;
                    self.ate = true;
                    self.map[p.x][p.y] = Cell::Blank;
                }
                _ => (),
            }
        }
        // Add new head
        self.map[p.x][p.y] = Cell::Body;
        self.snake.push_front(p);

        if !self.ate {
            if let Some(tail) = self.snake.pop_back() {
                self.map[tail.x][tail.y] = Cell::Blank;
            }
        }
        self.ate = false;
    }
}

fn main() {
    let mut game = Game::new(12);
    game.run();
}
