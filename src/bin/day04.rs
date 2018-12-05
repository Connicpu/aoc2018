use std::collections::HashMap;
use std::io::BufRead;
use std::io::Cursor;
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

fn parse_timestamp(input: &str) -> Option<Timestamp> {
    let mut split = input.split_whitespace();
    let date_part = split.next()?;
    let time_part = split.next()?;

    let mut split = date_part.split("-");
    let _year = split.next()?;
    let month = i32::from_str(split.next()?).ok()?;
    let day = i32::from_str(split.next()?).ok()?;

    let mut split = time_part.split(":");
    let hour = i32::from_str(split.next()?).ok()?;
    let minute = i32::from_str(split.next()?).ok()?;

    let date = Date { month, day };
    let time = Time { hour, minute };

    Some(Timestamp { date, time })
}

fn parse_action(input: &str) -> Option<Action> {
    let mut split = input.split(" ");
    Some(match split.next()? {
        "falls" => Action::Sleep,
        "wakes" => Action::Wake,
        "Guard" => {
            let id_part = split.next()?;
            let id = GuardId::from_str(&id_part[1..]).ok()?;
            Action::Shift(id)
        }
        _ => return None,
    })
}

fn parse_event(input: &str) -> Option<Event> {
    let mut split = input.split("]");
    let timestamp_part = split.next()?.trim_start_matches('[');
    let action_part = split.next()?;

    let timestamp = parse_timestamp(&timestamp_part[1..])?;
    let action = parse_action(action_part.trim())?;

    Some(Event { timestamp, action })
}

fn raw_events() -> impl Iterator<Item = Event> {
    static INPUT: &str = include_str!("day04.txt");
    Cursor::new(INPUT)
        .lines()
        .filter_map(|l| parse_event(&l.ok()?))
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
