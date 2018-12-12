type GridId = i32;

#[derive(Copy, Clone)]
struct Cell {
    x: usize,
    y: usize,
}

fn cell(x: usize, y: usize) -> Cell {
    Cell { x, y }
}

fn rack_id(cell: Cell) -> usize {
    cell.x + 10
}

fn power_level(grid: GridId, cell: Cell) -> i32 {
    let rid = rack_id(cell) as i32;
    let mut level = rid * cell.y as i32;
    level += grid;
    level *= rid;
    level %= 1000;
    level /= 100;
    level - 5
}

const GRID_SIZE: usize = 300;
const TOTAL_POS: usize = GRID_SIZE * GRID_SIZE;

fn all_valid_coords(size: usize) -> impl Iterator<Item = Cell> {
    (0..=GRID_SIZE - size).flat_map(move |x| (0..=GRID_SIZE - size).map(move |y| cell(x, y)))
}

#[derive(Clone)]
struct CellPowerMap {
    data: [i32; TOTAL_POS],
}

impl CellPowerMap {
    fn new() -> CellPowerMap {
        CellPowerMap { data: [0; TOTAL_POS] }
    }
}

impl std::ops::Index<usize> for CellPowerMap {
    type Output = [i32];
    fn index(&self, idx: usize) -> &[i32] {
        let bgn = idx * GRID_SIZE;
        let end = bgn + GRID_SIZE;
        &self.data[bgn..end]
    }
}

impl std::ops::Index<Cell> for CellPowerMap {
    type Output = i32;
    fn index(&self, idx: Cell) -> &i32 {
        &self.data[idx.y + GRID_SIZE * idx.x]
    }
}

impl std::ops::IndexMut<Cell> for CellPowerMap {
    fn index_mut(&mut self, idx: Cell) -> &mut i32 {
        &mut self.data[idx.y + GRID_SIZE * idx.x]
    }
}

const INPUT: GridId = 9445;

struct Result {
    power: i32,
    level: usize,
    pos: Cell,
}

fn power(g: &CellPowerMap, pos: Cell, s: usize) -> i32 {
    match (pos.x, pos.y) {
        (0, 0) => g[s][s],
        (0, y) => g[s][y + s] - g[s][y - 1],
        (x, 0) => g[x + s][s] - g[x - 1][s],
        (x, y) => g[x + s][y + s] - (g[x - 1][y + s] + g[x + s][y - 1] - g[x - 1][y - 1]),
    }
}

fn fill(grid_id: GridId, g: &mut CellPowerMap) {
    for pos in all_valid_coords(1) {
        g[pos] = -power(g, pos, 0) + power_level(grid_id, pos);
    }
}

fn scan(level: usize, g: &CellPowerMap, best: &mut Result) {
    for pos in all_valid_coords(level + 1) {
        let power = power(g, pos, level);
        if power > best.power {
            best.power = power;
            best.level = level;
            best.pos = pos;
        }
    }
}

fn main() {
    let mut g = CellPowerMap::new();
    fill(INPUT, &mut g);
    
    let mut best = Result {
        power: 0,
        level: 0,
        pos: cell(0, 0),
    };
    
    scan(2, &g, &mut best);
    println!("Part 1: {},{}", best.pos.x, best.pos.y);

    for i in 3..300 {
        scan(i, &g, &mut best);
    }
    println!("Part 2: {},{},{}", best.pos.x, best.pos.y, best.level + 1);
}
