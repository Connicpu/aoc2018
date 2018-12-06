use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::i32::{MAX, MIN};
use std::str::FromStr;

static INPUT: &str = include_str!("day06.txt");

type Coord = (i32, i32);

fn parse_coord(input: &str) -> Option<Coord> {
    let mut split = input.split(',');
    let x = i32::from_str(split.next()?.trim()).ok()?;
    let y = i32::from_str(split.next()?.trim()).ok()?;
    Some((x, y))
}

fn coords() -> impl Iterator<Item = Coord> {
    INPUT.split('\n').filter_map(|l| parse_coord(l))
}

fn manhattan(a: Coord, b: Coord) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

const BAD_COORD: Coord = (MIN / 8, MIN / 8);

fn closest_coord(pos: Coord) -> Coord {
    let mut closest = BAD_COORD;
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
        BAD_COORD
    } else {
        closest
    }
}

fn bounds() -> (Coord, Coord) {
    let mut tl = (MAX, MAX);
    let mut br = (MIN, MIN);
    for c in coords() {
        tl.0 = min(tl.0, c.0);
        tl.1 = min(tl.1, c.1);
        br.0 = max(br.0, c.0);
        br.1 = max(br.1, c.1);
    }
    (tl, br)
}

fn total_dist(pos: Coord) -> i32 {
    coords().map(|c| manhattan(c, pos)).sum()
}

fn main() {
    let ((left, top), (right, bottom)) = bounds();
    let mut counts = HashMap::<Coord, usize>::new();
    let mut inf_blacklist = HashSet::new();
    for y in top - 1..=bottom + 1 {
        for x in left - 1..=right + 1 {
            let id = closest_coord((x, y));
            *counts.entry(id).or_default() += 1;
            if y == top - 1 || y == bottom + 1 || x == left - 1 || x == right + 1 {
                inf_blacklist.insert(id);
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
            let c = (x, y);
            if total_dist(c) < 10_000 {
                viable += 1;
            }
        }
    }

    println!("Viable safe: {}", viable);
}
