use std::collections::VecDeque;

const PLAYERS: usize = 447;
const LAST_MARBLE_VALUE: usize = 71510 * 100;

type Marble = usize;

fn marbles() -> impl Iterator<Item = Marble> {
    0..=LAST_MARBLE_VALUE
}

fn rotate(amt: isize, circle: &mut VecDeque<Marble>) {
    if amt >= 0 {
        for _ in 0..amt {
            let temp = circle.pop_back().unwrap();
            circle.push_front(temp);
        }
    } else {
        for _ in 0..-amt {
            let temp = circle.pop_front().unwrap();
            circle.push_back(temp);
        }
    }
}

fn game(scores: &mut [usize; PLAYERS]) -> Option<()> {
    let mut circle = VecDeque::new();
    circle.push_back(0);
    circle.push_back(1);

    let mut marbles = marbles().skip(2);

    loop {
        for elf_score in scores.iter_mut() {
            let marble = marbles.next()?;
            if marble % 23 == 0 {
                rotate(7, &mut circle);
                *elf_score += marble + circle.pop_back().unwrap();
                rotate(-1, &mut circle);
            } else {
                rotate(-1, &mut circle);
                circle.push_back(marble);
            }
        }
    }
}

fn main() {
    let mut scores = [0; PLAYERS];
    game(&mut scores);
    println!("highscore: {}", scores.iter().cloned().max().unwrap());
}
