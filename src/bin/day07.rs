#![feature(range_contains)]

use aoc2018::{extract_columns, get_columns};
use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("day07.txt");

struct Req {
    id: char,
    dep: char,
}

fn requirements() -> impl Iterator<Item = Req> {
    INPUT
        .lines()
        .map(|line| get_columns(line, |c| !c.is_ascii_uppercase()))
        .filter_map(extract_columns![(x, x, x)])
        .filter_map(|(_, dep, id)| Some((dep.chars().next()?, id.chars().next()?)))
        .map(|(dep, id)| Req { dep, id })
}

#[derive(Debug)]
struct Graph {
    open: HashSet<char>,
    assigned: HashSet<char>,
    reqs: HashMap<char, HashSet<char>>,
}

fn can_exec(id: char, graph: &Graph) -> bool {
    if !graph.open.contains(&id) {
        return false;
    }

    if !graph.reqs.contains_key(&id) {
        return true;
    }

    graph.reqs[&id].intersection(&graph.open).count() == 0
}

fn make_graph() -> Graph {
    let assigned = HashSet::new();
    let mut open = HashSet::new();
    let mut reqs: HashMap<char, HashSet<char>> = HashMap::new();

    for req in requirements() {
        open.insert(req.id);
        open.insert(req.dep);
        reqs.entry(req.id).or_default().insert(req.dep);
    }

    Graph {
        open,
        reqs,
        assigned,
    }
}

fn assign_step(graph: &mut Graph) -> Option<(u32, char)> {
    for b in b'A'..=b'Z' {
        let c = b as char;
        if graph.assigned.contains(&c) {
            continue;
        }
        if can_exec(c, &graph) {
            graph.assigned.insert(c);
            //println!("Assigned {}", c);
            return Some(((c as u32 - 'A' as u32) + 61, c));
        }
    }
    None
}

const NUM_ELVES: usize = 5;

#[derive(Default)]
struct ElfPool {
    available: [u32; NUM_ELVES],
    task: [char; NUM_ELVES],
}

fn next_available(pool: &ElfPool) -> Option<usize> {
    for (elf, &time) in pool.available.iter().enumerate() {
        if time == 0 {
            return Some(elf);
        }
    }
    None
}

fn complete_task(task: char, graph: &mut Graph) {
    graph.assigned.remove(&task);
    graph.open.remove(&task);
}

fn tick(pool: &mut ElfPool, graph: &mut Graph) {
    for (elf, time) in pool.available.iter_mut().enumerate() {
        if *time > 0 {
            *time -= 1;
            if *time == 0 {
                complete_task(pool.task[elf], graph);
            }
        }
    }
}

fn is_complete(pool: &ElfPool, graph: &Graph) -> bool {
    pool.available.iter().all(|&t| t == 0) && graph.open.is_empty()
}

fn part1() {
    let mut graph = make_graph();
    while let Some((_, task)) = assign_step(&mut graph) {
        complete_task(task, &mut graph);
        print!("{}", task);
    }
    println!();
}

fn part2() {
    let mut graph = make_graph();
    let mut pool = ElfPool::default();

    let mut ticks = 0;
    loop {
        while let Some(elf) = next_available(&pool) {
            if let Some((time, task)) = assign_step(&mut graph) {
                pool.available[elf] = time;
                pool.task[elf] = task;
            } else {
                break;
            }
        }

        tick(&mut pool, &mut graph);
        ticks += 1;

        if is_complete(&pool, &graph) {
            break;
        }
    }

    println!("Ticks: {}", ticks);
}

fn main() {
    part1();
    part2();
}
