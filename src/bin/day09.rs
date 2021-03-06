use std::collections::VecDeque;

const PLAYERS: usize = 447;

type Marble = usize;

fn marbles(max: usize) -> impl Iterator<Item = Marble> {
    0..=max
}

#[derive(Default)]
struct Circle {
    marbles: VecDeque<Marble>,
}

impl Circle {
    fn rotate(&mut self, amt: isize) {
        if amt >= 0 {
            for _ in 0..amt {
                let temp = self.marbles.pop_back().unwrap();
                self.marbles.push_front(temp);
            }
        } else {
            for _ in 0..-amt {
                let temp = self.marbles.pop_front().unwrap();
                self.marbles.push_back(temp);
            }
        }
    }

    fn push(&mut self, marble: Marble) {
        self.marbles.push_back(marble);
    }

    fn pop(&mut self) -> Marble {
        self.marbles.pop_back().unwrap()
    }
}

fn game(scores: &mut [usize; PLAYERS], max: usize) -> Option<()> {
    let mut circle = Circle::default();
    circle.push(0);
    circle.push(1);

    let mut marbles = marbles(max).skip(2);

    loop {
        for elf_score in scores.iter_mut() {
            let marble = marbles.next()?;
            if marble % 23 == 0 {
                circle.rotate(7);
                *elf_score += marble + circle.pop();
                circle.rotate(-1);
            } else {
                circle.rotate(-1);
                circle.push(marble);
            }
        }
    }
}

fn highscore(max: usize) -> usize {
    let mut scores = [0; PLAYERS];
    game(&mut scores, max);
    scores.iter().cloned().max().unwrap()
}

fn main() {
    const LAST_MARBLE_VALUE: usize = 71510;

    println!("Highscore: {}", highscore(LAST_MARBLE_VALUE));
    println!("100x Highscore: {}", highscore(LAST_MARBLE_VALUE * 100));
}
