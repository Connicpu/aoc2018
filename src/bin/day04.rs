use aoc2018::{extract_columns, parse_columns};

use std::collections::HashMap;
use std::str::FromStr;

type GuardId = i32;

#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct Date {
    // year: 1518
    month: i32,
    day: i32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Time {
    hour: i32,
    minute: i32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Timestamp {
    date: Date,
    time: Time,
}

#[derive(Copy, Clone)]
enum Action {
    Shift(GuardId),
    Sleep,
    Wake,
}

#[derive(Copy, Clone)]
struct Event {
    timestamp: Timestamp,
    action: Action,
}

enum ParseElem {
    Falls,
    Asleep,
    Wakes,
    Up,
    Guard,
    Num(i32),
}

impl ParseElem {
    fn as_num(&self) -> i32 {
        match *self {
            ParseElem::Num(n) => n,
            _ => panic!(),
        }
    }
}

impl FromStr for ParseElem {
    type Err = ();
    fn from_str(s: &str) -> Result<ParseElem, ()> {
        use self::ParseElem::*;
        Ok(match s {
            "falls" => Falls,
            "asleep" => Asleep,
            "wakes" => Wakes,
            "up" => Up,
            "Guard" => Guard,
            _ => match s.parse() {
                Ok(num) => ParseElem::Num(num),
                Err(_) => return Err(()),
            },
        })
    }
}

fn parse_timestamp(elems: &[ParseElem]) -> Timestamp {
    let month = elems[0].as_num();
    let day = elems[1].as_num();
    let hour = elems[2].as_num();
    let minute = elems[3].as_num();

    Timestamp {
        date: Date { month, day },
        time: Time { hour, minute },
    }
}

fn parse_action(elems: &[ParseElem]) -> Action {
    match elems[0] {
        ParseElem::Falls => Action::Sleep,
        ParseElem::Wakes => Action::Wake,
        ParseElem::Guard => Action::Shift(elems[1].as_num()),
        _ => unreachable!()
    }
}

fn raw_events() -> impl Iterator<Item = Event> {
    static INPUT: &str = include_str!("day04.txt");
    INPUT
        .lines()
        .map(|line| parse_columns(line, |c| ":#-[ ]".contains(c)))
        .filter_map(extract_columns!([y, m, d, hour, min, action, id]))
        .map(|data| (parse_timestamp(&data[1..5]), parse_action(&data[5..7])))
        .map(|(timestamp, action)| Event { timestamp, action })
}

fn events() -> Vec<Event> {
    let mut events: Vec<Event> = raw_events().collect();
    events.sort_by_key(|e| e.timestamp);
    events
}

struct GuardSchedule {
    time_asleep: i32,
    asleep_times: [i32; 60],
}

impl GuardSchedule {
    fn best_minute(&self) -> i32 {
        self.asleep_times
            .iter()
            .enumerate()
            .max_by_key(|(_, &t)| t)
            .unwrap()
            .0 as i32
    }

    fn max_minute(&self) -> i32 {
        self.asleep_times[self.best_minute() as usize]
    }
}

impl Default for GuardSchedule {
    fn default() -> GuardSchedule {
        GuardSchedule {
            time_asleep: 0,
            asleep_times: [0; 60],
        }
    }
}

fn parse_sleep_schedule(events: &[Event]) -> HashMap<GuardId, GuardSchedule> {
    let mut sleeps = HashMap::<GuardId, GuardSchedule>::new();

    let mut guard_id = -1;
    let mut last_sleep = -1;

    for event in events {
        match event.action {
            Action::Shift(guard) => guard_id = guard,
            Action::Sleep => last_sleep = event.timestamp.time.minute,
            Action::Wake => {
                let schedule = sleeps.entry(guard_id).or_default();
                let now = event.timestamp.time.minute;

                schedule.time_asleep += now - last_sleep;
                for m in last_sleep..now {
                    schedule.asleep_times[m as usize] += 1;
                }
            }
        }
    }

    sleeps
}

fn main() {
    let events = events();
    let schedule = parse_sleep_schedule(&events);

    println!("Part 1:");

    let most_asleep = schedule
        .iter()
        .max_by_key(|(_, sched)| sched.time_asleep)
        .unwrap();
    let best_min = most_asleep.1.best_minute();

    println!(
        "{}*{} = {}",
        best_min,
        most_asleep.0,
        best_min * most_asleep.0
    );

    println!("Part 2:");

    let max_asleep = schedule
        .iter()
        .max_by_key(|(_, sched)| sched.max_minute())
        .unwrap();
    let best_min = max_asleep.1.best_minute();

    println!(
        "{}*{} = {}",
        best_min,
        max_asleep.0,
        best_min * max_asleep.0
    );
}
