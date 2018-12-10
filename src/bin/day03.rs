use aoc2018::{extract_columns, parse_columns};

use std::collections::HashSet;

use math2d::Recti;

static INPUT: &str = include_str!("day03.txt");

type Id = i32;

fn parse_claims() -> Vec<(Id, Recti)> {
    INPUT
        .lines()
        .map(|line| parse_columns(line, |c| !char::is_numeric(c)))
        .filter_map(extract_columns![(x, x, x, x, x)])
        .map(|(id, x, y, w, h)| (id, Recti::new(x, y, x + w, y + h)))
        .collect()
}

fn main() {
    let claims = parse_claims();

    let mut claimed = HashSet::new();
    let mut overlapped = HashSet::new();

    for (_, rect) in claims.iter() {
        for y in rect.top..rect.bottom {
            for x in rect.left..rect.right {
                if !claimed.insert((x, y)) {
                    overlapped.insert((x, y));
                }
            }
        }
    }

    println!("Overlapped inches: {}", overlapped.len());

    'claims: for (id, rect) in claims.iter() {
        for y in rect.top..rect.bottom {
            for x in rect.left..rect.right {
                if overlapped.contains(&(x, y)) {
                    continue 'claims;
                }
            }
        }

        println!("Nonoverlapped: {}", id);
    }
}
