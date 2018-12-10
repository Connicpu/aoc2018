use std::collections::HashSet;

static INPUT: &[i32] = &include!("day01.txt");

fn part_1() {
    let shift: i32 = INPUT.iter().sum();
    println!("Shift: {}", shift);
}

fn part_2() {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(0);

    let mut shift = 0;
    loop {
        for &value in INPUT {
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
