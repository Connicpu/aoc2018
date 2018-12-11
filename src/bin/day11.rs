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

const WIDTH: usize = 300;
const HEIGHT: usize = 300;

fn all_valid_coords(size: usize) -> impl Iterator<Item = Cell> {
    (0..=HEIGHT - size).flat_map(move |y| (0..=WIDTH - size).map(move |x| cell(x, y)))
}

#[derive(Clone)]
struct CellPowerMap {
    data: Box<[i32]>,
    width: usize,
}

impl CellPowerMap {
    fn new(level: usize) -> CellPowerMap {
        let size = 300 - level;
        CellPowerMap {
            data: vec![0; size * size].into_boxed_slice(),
            width: size,
        }
    }
}

impl std::ops::Index<Cell> for CellPowerMap {
    type Output = i32;
    fn index(&self, idx: Cell) -> &i32 {
        &self.data[idx.x + self.width * idx.y]
        //unsafe { self.data.get_unchecked(idx.x + self.width * idx.y) }
    }
}

impl std::ops::IndexMut<Cell> for CellPowerMap {
    fn index_mut(&mut self, idx: Cell) -> &mut i32 {
        &mut self.data[idx.x + self.width * idx.y]
        //unsafe { self.data.get_unchecked_mut(idx.x + self.width * idx.y) }
    }
}

fn partial_power_cmp(cache: &CellPowerMap, level: usize, partial: i32, pos: Cell) -> i32 {
    let mut total = partial;
    let x = pos.x + level;
    for y in pos.y..pos.y + level {
        total += cache[cell(x, y)];
    }
    let y = pos.y + level;
    for x in pos.x..=pos.x + level {
        total += cache[cell(x, y)];
    }
    total
}

const INPUT: GridId = 9445;

struct Result {
    power: i32,
    level: usize,
    pos: Cell,
}

fn prime_cache(grid: GridId, power_levels: &mut CellPowerMap) {
    for pos in all_valid_coords(1) {
        let power = power_level(grid, pos);
        power_levels[pos] = power;
    }
}

#[inline(always)]
fn calculate_level(
    level: usize,
    cache: &CellPowerMap,
    power_levels: &mut CellPowerMap,
    best: &mut Result,
) {
    for pos in all_valid_coords(level + 1) {
        let partial = power_levels[pos];
        let power = partial_power_cmp(cache, level, partial, pos);
        if power > best.power {
            best.power = power;
            best.pos = pos;
            best.level = level;
        }
        power_levels[pos] = power;
    }
}

fn main() {
    let grid = INPUT;

    let mut best = Result {
        power: 0,
        level: 0,
        pos: cell(0, 0),
    };

    let mut power_levels = CellPowerMap::new(0);
    prime_cache(grid, &mut power_levels);

    let cache = power_levels.clone();
    calculate_level(1, &cache, &mut power_levels, &mut best);
    calculate_level(2, &cache, &mut power_levels, &mut best);

    println!("Part 1: {},{},{}", best.pos.x, best.pos.y, best.level + 1);

    for level in 3..300 {
        calculate_level(level, &cache, &mut power_levels, &mut best);
    }

    println!("Part 2: {},{},{}", best.pos.x, best.pos.y, best.level + 1);
}
