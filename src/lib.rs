use cell::Cell;
use game_hardness::GameHardness;
use game_status::GameStatus;
use position::Position;
use rand::Rng;
use std::{collections::VecDeque, process};

mod cell;
mod game_hardness;
mod game_status;
mod position;

pub struct Game {
    // Map information
    map: Vec<Vec<Cell>>,
    food: Position,
    // Runtime information
    status: GameStatus,
    timestamp: u32, // The process of the Game
    score: u32,     // The length of snake (l) is l = score + 5;
    length: u32,

    // Snake information
    snake: VecDeque<Position>,
    ate: bool,
    last_direction: char,

    // Configuration
    hardness: GameHardness,
    map_size: usize,
}

impl Game {
    pub fn new(map_size: usize) -> Game {
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
            food: Position::new(),

            timestamp: 0,
            score: 0,
            length,
            status: GameStatus::Initialize,

            snake,
            ate: false,
            last_direction: 'l',

            hardness: GameHardness::Normal,
            map_size,
        }
    }
    /**
    Run the game.
    */
    pub fn run(&mut self) {
        self.status = GameStatus::Running;
        let size = self.map_size as u32;
        let stdin = std::io::stdin();
        loop {
            if self.length == size * size {
                self.win();
            }

            if self.timestamp % 20 == 0 {
                // generate a new food due to time.
                // 1. clean the old food from map
                let old_food = &self.food;
                self.map[old_food.x][old_food.y] = Cell::Blank;
                // 2. generate a new food
                self.generate_food();
            };
            if self.food.x == self.map_size {
                // generate a new food due to ate.
                self.generate_food();
            }

            self.timestamp = self.timestamp + 1;

            self.display_map();
            // wait input
            let mut buf = String::new();
            stdin.read_line(&mut buf).expect("Failed reading input.");
            let direction: Vec<char> = buf.trim().chars().collect();
            let mut next_pos: Position = Position::new();
            if let Some(curr_head) = self.snake.front() {
                let mut d = self.last_direction;
                if direction.len() > 0 {
                    d = direction[direction.len() - 1];
                }
                let direction = match d {
                    'h' => 'h',
                    'j' => 'j',
                    'k' => 'k',
                    'l' => 'l',
                    _ => self.last_direction,
                };
                self.last_direction = direction;
                next_pos = curr_head
                    .get_next(direction, self.map_size)
                    .unwrap_or_else(|err| {
                        println!("{}", err);
                        self.lose();
                        Position::new() // never reach here
                    });
            }
            self.move_snake(next_pos);
        }
    }

    fn generate_food(&mut self) {
        loop {
            // TODO some bug here
            let x = rand::thread_rng().gen_range(0..self.map_size);
            let y = rand::thread_rng().gen_range(0..self.map_size);
            match self.map[x][y] {
                Cell::Blank => {
                    self.food = Position { x, y };
                    self.map[x][y] = Cell::Food;
                }
                _ => {
                    continue;
                }
            }
        }
    }

    fn win(&mut self) {
        self.status = GameStatus::Finished;
        println!(
            "\tCongratulations!\n\nYou won the game.\n map size: {}\n hardness: {}",
            self.map_size, self.hardness
        );
        process::exit(0);
    }

    fn lose(&mut self) {
        self.status = GameStatus::Finished;
        println!("\tGame Over!\n\n\tscore: {}", self.score);
        process::exit(0);
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
                    self.lose();
                }
                Cell::Body => {
                    self.lose();
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
