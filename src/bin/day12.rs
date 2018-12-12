#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2018::*;
use std::collections::*;

static INPUT: &str = include_str!("day12.txt");

fn initial_state() -> &'static [u8] {
    &INPUT.as_bytes()[..100]
}

fn patterns() -> impl Iterator<Item = (&'static [u8], u8)> {
    collect_once! {
        let data: (&'static [u8], u8) = INPUT
            .lines()
            .skip(2)
            .map(|line| line.split(" => "))
            .filter_map(extract_columns![(x, x)])
            .map(|(pat, res)| (pat.as_bytes(), res.as_bytes()[0]))
            .take(32)
    }
}

const POTTED: u8 = b'#';
const EMPTY: u8 = b'.';

#[derive(Default)]
struct PotRow {
    pots: VecDeque<u8>,
    next: VecDeque<u8>,
    center: usize,
    generation: isize,
}

impl PotRow {
    fn begin(&self) -> isize {
        -(self.center as isize) - 2
    }

    fn end(&self) -> isize {
        (self.pots.len() - self.center) as isize + 2
    }

    fn get(&self, idx: isize) -> u8 {
        let idx = self.center as isize + idx;
        if idx < 0 || idx >= self.pots.len() as isize {
            return EMPTY;
        }
        self.pots[idx as usize]
    }

    fn get_nxt(&self, idx: isize) -> u8 {
        let idx = self.center as isize + idx;
        if idx < 0 || idx >= self.pots.len() as isize {
            return EMPTY;
        }
        self.next[idx as usize]
    }

    fn pattern_at(&self, idx: isize) -> [u8; 5] {
        [
            self.get(idx - 2),
            self.get(idx - 1),
            self.get(idx),
            self.get(idx + 1),
            self.get(idx + 2),
        ]
    }

    fn set(&mut self, idx: isize, value: u8) {
        if idx < 0 {
            while -idx > self.center as isize {
                self.pots.push_front(EMPTY);
                self.next.push_front(EMPTY);
                self.center += 1;
            }
        } else {
            let absidx = self.center + idx as usize;
            while absidx >= self.pots.len() {
                self.pots.push_back(EMPTY);
                self.next.push_back(EMPTY);
            }
        }

        let idx = self.center as isize + idx;
        self.next[idx as usize] = value;
    }
}

fn fmt_pots<'a>(pots: impl IntoIterator<Item = &'a u8>) -> String {
    pots.into_iter().map(|&b| b as char).collect::<String>()
}

fn get_res(ipat: [u8; 5]) -> u8 {
    for (pat, res) in patterns() {
        if ipat == pat {
            return res;
        }
    }
    unreachable!("can't find '{}'", fmt_pots(&ipat));
}

fn get_total(state: &PotRow) -> isize {
    let mut total = 0;
    for i in state.begin()..state.end() {
        if state.get(i) == POTTED {
            total += i;
        }
    }
    total
}

fn process(state: &mut PotRow) {
    for i in state.begin()..state.end() {
        let ipat = state.pattern_at(i);
        let res = get_res(ipat);
        if res != state.get(i) {
            state.set(i, res);
        }
        assert_eq!(state.get_nxt(i), res, "{}", state.get(i));
    }

    state.pots.clone_from(&state.next);
    state.generation += 1;
}

fn reset_state(state: &mut PotRow) {
    state.pots.clear();
    state.pots.extend(initial_state());
    state.next.clone_from(&state.pots);
    state.center = 0;
    state.generation = 0;
}

fn part1(state: &mut PotRow) -> isize {
    while state.generation < 20 {
        process(state);
    }

    get_total(state)
}

fn part2(state: &mut PotRow) -> isize {
    while state.generation < 200 {
        process(state);
    }

    let g2k = get_total(state);
    process(state);
    let g2k1 = get_total(state);

    let gens_left = 50_000_000_000 - state.generation;
    let dsum = g2k1 - g2k;

    g2k1 + dsum * gens_left
}

fn main() {
    let mut state = PotRow::default();
    reset_state(&mut state);

    let total = part1(&mut state);
    println!("Part 1: {}", total);

    let total = part2(&mut state);
    println!("Part 2: {}", total);
}

#[cfg(test)]
mod bench {
    use test::Bencher;
    use super::{PotRow, reset_state, part1, part2};

    #[bench]
    fn bench_part1(bench: &mut Bencher) {
        let mut state = PotRow::default();
        bench.iter(|| {
            reset_state(&mut state);
            part1(&mut state)
        });
    }

    #[bench]
    fn bench_part2(bench: &mut Bencher) {
        let mut state = PotRow::default();
        bench.iter(|| {
            reset_state(&mut state);
            part2(&mut state)
        });
    }
}

