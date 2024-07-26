use crossterm::{
    cursor, execute, queue,
    style::{self},
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, stdout, Write},
    sync::{mpsc::Receiver, Arc, Mutex},
};

use super::Cell;

type Map = Vec<Vec<Cell>>;

pub fn display(rx: Receiver<Arc<Mutex<Map>>>, rows: usize, cols: usize) -> io::Result<()> {
    let mut stdout = stdout();
    let mut last_map = vec![vec![Cell::Blank; cols]; rows];

    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;
    queue!(
        stdout,
        style::ResetColor,
        terminal::Clear(ClearType::All),
        cursor::Hide,
    )?;
    for row in 0..rows {
        for col in 0..cols {
            queue!(
                stdout,
                cursor::MoveTo((col * 2) as u16, row as u16),
                style::Print('🐾')
            )?;
        }
    }
    stdout.flush()?;
    loop {
        match rx.recv() {
            Ok(map) => {
                let new_map = map.lock().unwrap();
                for row in 0..rows {
                    for col in 0..cols {
                        if new_map[row][col] != last_map[row][col] {
                            let c = new_map[row][col].clone();
                            queue!(
                                stdout,
                                cursor::MoveTo((col * 2) as u16, row as u16),
                                style::Print(match c {
                                    Cell::Blank => '🐾',
                                    Cell::Body => '🚌',
                                    Cell::Head => '👶',
                                    Cell::Food => '🍎',
                                })
                            )?;
                            last_map[row][col] = c;
                        }
                    }
                }
                stdout.flush()?;
            }
            Err(_) => {
                break;
            }
        }
    }
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)
}
