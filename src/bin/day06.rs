use aoc2018::{collect_once, extract_columns, parse_columns};

use std::collections::{HashMap, HashSet};
use std::i32::MAX;

use math2d::{Point2i, Recti};

static INPUT: &str = include_str!("day06.txt");

fn coords() -> impl Iterator<Item = Point2i> {
    collect_once! {
        let data: Point2i = INPUT
            .lines()
            .map(|line| parse_columns(line, |c| !char::is_numeric(c)))
            .filter_map(extract_columns![(x, x)])
            .map(|(x, y)| Point2i { x, y })
    }
}

fn manhattan(a: Point2i, b: Point2i) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn closest_coord(pos: Point2i) -> Option<Point2i> {
    let mut closest = Point2i::ORIGIN;
    let mut closest_dist = MAX;
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
    coords().fold(Recti::EMPTY, |r, c| r.combined_with(c))
}

fn total_dist(pos: Point2i) -> i32 {
    coords().map(|c| manhattan(c, pos)).sum()
}

fn main() {
    let bounds = bounds();

    let mut counts = HashMap::<Point2i, usize>::new();
    let mut inf_blacklist = HashSet::new();

    for c in bounds.points() {
        if let Some(id) = closest_coord(c) {
            *counts.entry(id).or_default() += 1;
            if bounds.is_on_edge(c) {
                inf_blacklist.insert(id);
            }
        }
    }

    for id in inf_blacklist.iter() {
        counts.remove(id);
    }

    println!("Biggest region: {}", counts.values().max().unwrap());

    let mut viable = 0;
    for c in bounds.points() {
        if total_dist(c) < 10_000 {
            viable += 1;
        }
    }

    println!("Viable safe: {}", viable);
}
