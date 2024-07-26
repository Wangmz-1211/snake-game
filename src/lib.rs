use cell::Cell;
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::size,
};
use display::display;
use game_status::GameStatus;
use position::Position;
use rand::Rng;
use std::{
    collections::VecDeque,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

mod cell;
mod display;
mod game_status;
mod position;

pub struct Game {
    // Map information
    map: Arc<Mutex<Vec<Vec<Cell>>>>,
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
    map_cols: usize,
    map_rows: usize,
}

impl Game {
    /// Generate a default setting for game.
    pub fn new() -> Game {
        // Initialize map
        // The minimum map_size is 5
        let (cols, rows) = size().unwrap();
        let map_cols = cols as usize / 2;
        let map_rows = rows as usize;
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
            map: Arc::new(Mutex::new(map)),
            food: Position::new(),
            food_time: 0,

            timestamp: 0,
            score: 0,
            length,
            status: GameStatus::Initialize,

            snake,
            ate: false,
            last_event: Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::empty())),

            map_cols,
            map_rows,
        }
    }

    /// Run the game
    pub fn run(&mut self) {
        self.status = GameStatus::Running;

        {
            // Prepare display thread
            let map_rows = self.map_rows;
            let map_cols = self.map_cols;
            let (tx, rx) = mpsc::channel();
            let _handle = thread::spawn(move || display(rx, map_rows, map_cols));
            loop {
                self.refresh_food_state();
                // 3. time
                self.timestamp = self.timestamp + 1;
                self.food_time -= 1;

                // 4. displaying (send map pointer to channel)
                {   // Preprocessing the map. Display snake head.              
                    // The postprocessing code is placed far from here to make
                    // more time for displaying thread.
                    let mut map = self.map.lock().unwrap();
                    let head = self.snake.front().unwrap();
                    map[head.x][head.y] = Cell::Head;
                }
                tx.send(Arc::clone(&self.map)).expect("Send map failed");

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

                { // Postprocessing for the displaying.
                    
                    let mut map = self.map.lock().unwrap();
                    let head = self.snake.front().unwrap();
                    map[head.x][head.y] = Cell::Body;
                }

                // get next position
                let next_pos = match self.get_next_pos(event) {
                    Ok(p) => p,
                    Err(_) => {
                        self.status = GameStatus::Finished;
                        break;
                    }
                };

                self.move_snake(next_pos);
                match self.status {
                    GameStatus::Finished => {
                        break;
                    }
                    _ => (),
                }
            }
        }
        thread::sleep(Duration::from_millis(1));
        let max_score = self.map_cols * self.map_rows - 3;
        println!("    Game Over\n\n You got {} / {}!", self.score, max_score);
    }

    fn get_next_pos(&mut self, event: Event) -> Result<Position, String> {
        self.snake
            .front()
            .unwrap()
            .get_next(event, self.map_rows, self.map_cols)
    }

    fn refresh_food_state(&mut self) {
        if self.food_time > 0 {
            return;
        }
        let old_food_pos = &self.food;
        let mut map = self.map.lock().unwrap();
        if map[old_food_pos.x][old_food_pos.y] == Cell::Food {
            map[old_food_pos.x][old_food_pos.y] = Cell::Blank;
        }
        loop {
            let x = rand::thread_rng().gen_range(0..self.map_rows);
            let y = rand::thread_rng().gen_range(0..self.map_cols);
            match map[x][y] {
                Cell::Blank => {
                    self.food = Position { x, y };
                    map[x][y] = Cell::Food;
                    self.food_time = 2 * (self.map_cols + self.map_rows) as u32;
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
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
        let curr_head = self.snake.front().unwrap();
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
        let mut map = self.map.lock().unwrap();
        let content = map[p.x][p.y].clone();
        match content {
            Cell::Body => {
                self.status = GameStatus::Finished;
                println!("\tGame Over!\n\n\tscore: {}", self.score);
            }
            Cell::Food => {
                self.score += 1;
                self.length += 1;
                self.ate = true;
                map[p.x][p.y] = Cell::Blank;
                self.food_time = 0;
            }
            _ => (),
        }
        // Add new head
        map[p.x][p.y] = Cell::Body;
        self.snake.push_front(p);

        if !self.ate {
            if let Some(tail) = self.snake.pop_back() {
                map[tail.x][tail.y] = Cell::Blank;
            }
        }
        self.ate = false;
    }
}
