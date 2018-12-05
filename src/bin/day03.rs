extern crate math2d;

use std::collections::HashSet;
use std::io::BufRead;
use std::io::Cursor;
use std::str::FromStr;

use math2d::Point2i;
use math2d::Recti;
use math2d::Vector2i;

static INPUT: &str = include_str!("day03.txt");

type Id = i32;

fn parse_pos(pos: &str) -> Option<Point2i> {
    let mut split = pos.split(":").nth(0)?.split(',');
    let x = i32::from_str(split.next()?).ok()?;
    let y = i32::from_str(split.next()?).ok()?;
    Some((x, y).into())
}

fn parse_size(size: &str) -> Option<Vector2i> {
    let mut split = size.split('x');
    let x = i32::from_str(split.next()?).ok()?;
    let y = i32::from_str(split.next()?).ok()?;
    Some([x, y].into())
}

fn parse_claim(claim: &str) -> Option<(Id, Recti)> {
    let mut split = claim.split_whitespace();

    let id = Id::from_str(&split.next()?[1..]).ok()?;
    let _ = split.next(); // @
    let pos = parse_pos(split.next()?)?;
    let size = parse_size(split.next()?)?;
    let br = pos + size;
    let rect = Recti::new(pos.x, pos.y, br.x, br.y);

    Some((id, rect))
}

fn parse_claims() -> Vec<(Id, Recti)> {
    Cursor::new(INPUT)
        .lines()
        .filter_map(|l| parse_claim(&l.unwrap()))
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
