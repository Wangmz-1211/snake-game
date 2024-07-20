use cell::Cell;
use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue, style,
    terminal::{self, disable_raw_mode, enable_raw_mode, size, ClearType},
};
use game_hardness::GameHardness;
use game_status::GameStatus;
use position::Position;
use rand::Rng;
use std::{
    collections::VecDeque,
    io::{self, stdout, Write},
    time::Duration,
};

mod cell;
mod game_hardness;
mod game_status;
mod position;

pub struct Game {
    // Map information
    map: Vec<Vec<Cell>>,
    food: Position,
    food_time: u32,
    // Runtime information
    status: GameStatus,
    timestamp: u32, // The process of the Game
    score: u32,     // The length of snake (l) is l = score + 5;
    length: u32,

    // Snake information
    snake: VecDeque<Position>,
    ate: bool,
    last_event: Event,

    // Configuration
    hardness: GameHardness,
    map_cols: usize,
    map_rows: usize,
}

impl Game {
    pub fn new() -> Game {
        // Initialize map
        // The minimum map_size is 5
        let (cols, rows) = size().unwrap();
        let map_cols = cols as usize / 2;
        let map_rows = rows as usize - 3;
        enable_raw_mode().unwrap();
        let mut map = vec![vec![Cell::Blank; map_cols]; map_rows];

        // Initialize snake
        // The init length of snake
        let x_mid = (map_rows + 1) / 2;
        let y_mid = (map_cols + 1) / 2;
        let length = 3;
        let mut snake = VecDeque::new();
        for i in 0..length {
            let y = y_mid - i as usize;
            // head is always at snake.front()
            snake.push_back(Position { x: x_mid, y });
            map[x_mid][y] = Cell::Body;
        }

        Game {
            map,
            food: Position::new(),
            food_time: 0,

            timestamp: 0,
            score: 0,
            length,
            status: GameStatus::Initialize,

            snake,
            ate: false,
            last_event: Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::empty())),

            hardness: GameHardness::Normal,
            map_cols,
            map_rows,
        }
    }
    /**
    Run the game.
    */
    pub fn run(&mut self) {
        self.status = GameStatus::Running;
        let mut w = stdout();
        execute!(w, terminal::EnterAlternateScreen).unwrap();
        loop {
            // 1. Check the player has won
            if self.length == (self.map_rows * self.map_cols) as u32 {
                self.status = GameStatus::Finished;
                println!(
                    "\tCongratulations!\n\nYou won the game.\n hardness: {}",
                    self.hardness
                );
                break;
            }

            if self.food_time == 0 {
                // generate a new food due to time.
                // 1. clean the old food from map
                let old_food = &self.food;
                self.map[old_food.x][old_food.y] = Cell::Blank;
                // 2. generate a new food
                self.generate_food();
            };

            // 3. time
            self.timestamp = self.timestamp + 1;
            self.food_time -= 1;

            // 4. Display map
            self.display(&mut w).unwrap();

            // 5. Input
            let mut event = self.last_event.clone();
            let wait_time = self.map_cols * self.map_rows - self.score as usize;
            if poll(Duration::from_millis(wait_time as u64)).unwrap() {
                event = read().unwrap();
            }
            if let Event::Key(key_event) = event {
                if key_event.code == KeyCode::Esc {
                    break;
                }
                match key_event.code {
                    KeyCode::Esc => {
                        self.status = GameStatus::Finished;
                        break;
                    }
                    KeyCode::Up => {
                        if let Event::Key(last_event) = self.last_event {
                            if last_event.code != KeyCode::Down {
                                self.last_event = event.clone();
                            } else {
                                event = self.last_event.clone();
                            }
                        }
                    }
                    KeyCode::Left => {
                        if let Event::Key(last_event) = self.last_event {
                            if last_event.code != KeyCode::Right {
                                self.last_event = event.clone();
                            } else {
                                event = self.last_event.clone();
                            }
                        }
                    }
                    KeyCode::Down => {
                        if let Event::Key(last_event) = self.last_event {
                            if last_event.code != KeyCode::Up {
                                self.last_event = event.clone();
                            } else {
                                event = self.last_event.clone();
                            }
                        }
                    }
                    KeyCode::Right => {
                        if let Event::Key(last_event) = self.last_event {
                            if last_event.code != KeyCode::Left {
                                self.last_event = event.clone();
                            } else {
                                event = self.last_event.clone();
                            }
                        }
                    }
                    _ => {
                        event = self.last_event.clone();
                    }
                }
            }

            let mut next_pos: Position = Position::new();
            if let Some(curr_head) = self.snake.front() {
                next_pos = curr_head
                    .get_next(event, self.map_rows, self.map_cols)
                    .unwrap_or_else(|err| {
                        queue!(w, style::Print(err)).unwrap();
                        w.flush().unwrap();
                        self.status = GameStatus::Finished;
                        Position::new() // never reach here
                    });
            }
            match self.status {
                GameStatus::Finished => {
                    break;
                }
                _ => (),
            }
            self.move_snake(next_pos);
            match self.status {
                GameStatus::Finished => {
                    break;
                }
                _ => (),
            }
        }
        execute!(
            w,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )
        .unwrap();
        disable_raw_mode().unwrap();

        let max_score = self.map_cols * self.map_rows - 3;
        println!("    Game Over\n\n You got {} / {}!", self.score, max_score);
    }

    fn generate_food(&mut self) {
        loop {
            let x = rand::thread_rng().gen_range(0..self.map_rows);
            let y = rand::thread_rng().gen_range(0..self.map_cols);
            match self.map[x][y] {
                Cell::Blank => {
                    self.food = Position { x, y };
                    self.map[x][y] = Cell::Food;
                    self.food_time = 2 * (self.map_cols + self.map_rows) as u32;
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
    }

    fn display<W>(&mut self, w: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        // tmply replace head for display
        let head_pos = match self.snake.front() {
            Some(pos) => pos,
            None => &Position::new(),
        };
        self.map[head_pos.x][head_pos.y] = Cell::Head;

        // 1. generate strs first
        let game_info = format!("\tScore: {}", self.score);
        let mut map_lines: Vec<String> = vec![String::new(); self.map_rows];
        for i in 0..self.map_rows {
            map_lines[i] = self.map[i]
                .iter()
                .map(|cell| match cell {
                    Cell::Blank => '🐾',
                    Cell::Wall => '🧱',
                    Cell::Food => '🍎',
                    Cell::Body => '🚌',
                    Cell::Head => '👶',
                })
                .collect();
        }
        self.map[head_pos.x][head_pos.y] = Cell::Body;
        // 2. display
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(1, 1)
        )?;
        queue!(w, style::Print(game_info), cursor::MoveToNextLine(2))?; // title
        for line in map_lines.iter() {
            queue!(w, style::Print(line), cursor::MoveToNextLine(1))?; // map
        }
        w.flush().unwrap();

        Ok(())
    }

    /// Move the snake head to the next block,
    /// Also handle the `ate` buff.
    fn move_snake(&mut self, p: Position) {
        // check position valid
        if p.x >= self.map_rows || p.y >= self.map_cols {
            println!("Hit Wall! Game Over.");
            self.status = GameStatus::Finished;
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
                    self.status = GameStatus::Finished;
                    println!("\tGame Over!\n\n\tscore: {}", self.score);
                }
                Cell::Body => {
                    self.status = GameStatus::Finished;
                    println!("\tGame Over!\n\n\tscore: {}", self.score);
                }
                Cell::Food => {
                    self.score += 1;
                    self.length += 1;
                    self.ate = true;
                    self.map[p.x][p.y] = Cell::Blank;
                    self.food_time = 0;
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
