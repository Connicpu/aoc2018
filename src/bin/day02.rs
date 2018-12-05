use std::collections::BTreeMap;
use std::io::BufRead;
use std::io::Cursor;

static INPUT: &str = include_str!("day02.txt");

fn box_value(id: &str, repeats: &mut BTreeMap<char, i32>) -> (bool, bool) {
    repeats.clear();

    let mut twos = 0;
    let mut threes = 0;

    for c in id.chars() {
        let entry = repeats.entry(c).or_default();

        match *entry {
            2 => twos -= 1,
            3 => threes -= 1,
            _ => (),
        }

        *entry += 1;

        match *entry {
            2 => twos += 1,
            3 => threes += 1,
            _ => (),
        }
    }

    (twos > 0, threes > 0)
}

fn part_1() {
    let mut repeats = BTreeMap::new();
    let mut lines = Cursor::new(INPUT).lines();

    let mut twos = 0u64;
    let mut threes = 0u64;

    while let Some(Ok(line)) = lines.next() {
        let (has_2, has_3) = box_value(&line, &mut repeats);
        if has_2 {
            twos += 1;
        }
        if has_3 {
            threes += 1;
        }
    }

    println!("Checksum: {}", twos * threes);
}

fn box_diff(box1: &str, box2: &str) -> i32 {
    let mut diff = 0;
    for (c1, c2) in box1.chars().zip(box2.chars()) {
        if c1 != c2 {
            diff += 1;
        }
    }
    diff
}

fn box_common(box1: &str, box2: &str) -> String {
    let mut result = String::with_capacity(box1.len());
    for (c1, c2) in box1.chars().zip(box2.chars()) {
        if c1 == c2 {
            result.push(c1);
        }
    }
    result
}

fn part_2() {
    let mut lines = Cursor::new(INPUT).lines();
    while let Some(Ok(box1)) = lines.next() {
        let mut lines = Cursor::new(INPUT).lines();
        while let Some(Ok(box2)) = lines.next() {
            if box_diff(&box1, &box2) == 1 {
                println!("{} and {}", box1, box2);
                println!("common: {}", box_common(&box1, &box2));
                return;
            }
        }
    }
}

fn main() {
    println!("Part 1:");
    part_1();
    println!("Part 2:");
    part_2();
}
