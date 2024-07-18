#[derive(Clone, Debug)]
enum Cell {
    Wall,
    Food,
    Body,
    Blank,
}



fn initialize_map(size: usize) -> Vec<Vec<Cell>> {
    let map: Vec<Vec<Cell>> = vec![vec![Cell::Blank; size]; size];

    map
}

fn main() {
    let map = initialize_map(3);
    println!("{:?}", map);
}
