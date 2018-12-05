use std::collections::HashSet;
use std::io::BufRead;
use std::io::Cursor;
use std::str::FromStr;

static INPUT: &str = include_str!("day01.txt");

fn part_1() {
    let mut lines = Cursor::new(INPUT).lines();
    let mut shift = 0;
    while let Some(Ok(line)) = lines.next() {
        if line.trim() == "" {
            continue;
        }

        let value: i32 = FromStr::from_str(&line).unwrap();
        shift += value;
    }
    println!("Shift: {}", shift);
}

fn part_2() {
    let mut shift = 0;

    let mut set: HashSet<i32> = HashSet::new();
    set.insert(0);

    loop {
        let mut lines = Cursor::new(INPUT).lines();
        while let Some(Ok(line)) = lines.next() {
            if line.trim() == "" {
                continue;
            }

            let value: i32 = FromStr::from_str(&line).unwrap();
            shift += value;

            if set.contains(&shift) {
                println!("Reached {} twice", shift);
                return;
            }
            set.insert(shift);
        }
    }
}

fn main() {
    println!("Part 1:");
    part_1();

    println!("Part 2:");
    part_2();
}
