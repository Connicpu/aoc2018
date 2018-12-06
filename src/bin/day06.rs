use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::i32::{MAX, MIN};
use std::str::FromStr;

use math2d::Recti;

static INPUT: &str = include_str!("day06.txt");

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }
}

fn parse_coord(input: &str) -> Option<Coord> {
    let mut split = input.split(',');
    let x = i32::from_str(split.next()?.trim()).ok()?;
    let y = i32::from_str(split.next()?.trim()).ok()?;
    Some(Coord::new(x, y))
}

fn coords() -> impl Iterator<Item = Coord> {
    INPUT.split('\n').filter_map(|l| parse_coord(l))
}

fn manhattan(a: Coord, b: Coord) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn closest_coord(pos: Coord) -> Option<Coord> {
    let mut closest = Coord::new(0, 0);
    let mut closest_dist = manhattan(closest, pos);
    let mut closest_count = 0;
    for c in coords() {
        let dist = manhattan(c, pos);
        if dist < closest_dist {
            closest = c;
            closest_dist = dist;
            closest_count = 1;
        } else if closest_dist == dist {
            closest_count += 1;
        }
    }

    if closest_count > 1 {
        None
    } else {
        Some(closest)
    }
}

fn bounds() -> Recti {
    let mut rect = Recti::new(MAX, MAX, MIN, MIN);
    for c in coords() {
        rect.left = min(rect.left, c.x);
        rect.top = min(rect.top, c.y);
        rect.right = max(rect.right, c.x);
        rect.bottom = max(rect.bottom, c.y);
    }
    rect
}

fn total_dist(pos: Coord) -> i32 {
    coords().map(|c| manhattan(c, pos)).sum()
}

fn main() {
    let Recti {
        left,
        top,
        right,
        bottom,
    } = bounds();

    let mut counts = HashMap::<Coord, usize>::new();
    let mut inf_blacklist = HashSet::new();

    for y in top - 1..=bottom + 1 {
        for x in left - 1..=right + 1 {
            let c = Coord::new(x, y);
            if let Some(id) = closest_coord(c) {
                *counts.entry(id).or_default() += 1;
                if y == top - 1 || y == bottom + 1 || x == left - 1 || x == right + 1 {
                    inf_blacklist.insert(id);
                }
            }
        }
    }

    for id in inf_blacklist.iter() {
        counts.remove(id);
    }

    println!("Biggest region: {}", counts.values().max().unwrap());

    let mut viable = 0;
    for y in top..=bottom {
        for x in left..=right {
            let c = Coord::new(x, y);
            if total_dist(c) < 10_000 {
                viable += 1;
            }
        }
    }

    println!("Viable safe: {}", viable);
}
