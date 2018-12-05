static INPUT: &str = include_str!("day05.txt");

#[derive(Clone)]
struct Polymer {
    data: Vec<u8>,
}

impl Polymer {
    fn new(data: &str) -> Polymer {
        Polymer {
            data: Vec::from(data.trim()),
        }
    }

    fn react(&mut self) {
        let mut i = 0;
        while self.data.len() > 0 && i < self.data.len() - 1 {
            let (left, right) = Unit::get(&self.data, i);
            if left.cancels(right) {
                self.data.drain(i..i + 2);
                if i > 0 {
                    i -= 1;
                }
            } else {
                i += 1;
            }
        }
    }
}

#[derive(Copy, Clone)]
struct Unit {
    id: u8,
    polarity: bool,
}

impl Unit {
    fn new(data: u8) -> Unit {
        Unit {
            id: data.to_ascii_uppercase(),
            polarity: data.is_ascii_uppercase(),
        }
    }

    fn get(data: &[u8], i: usize) -> (Unit, Unit) {
        let left = Unit::new(data[i]);
        let right = Unit::new(data[i + 1]);
        (left, right)
    }

    fn cancels(self, rhs: Unit) -> bool {
        self.id == rhs.id && self.polarity != rhs.polarity
    }
}

fn main() {
    println!("Part 1:");
    let mut polymer = Polymer::new(INPUT);
    polymer.react();
    println!("Remaining: {}", polymer.data.len());

    println!("Part 2:");
    let shortest = (b'A'..=b'Z')
        .map(|a| {
            let mut polymer = Polymer::new(INPUT);
            polymer.data.retain(|&b| Unit::new(b).id != a);
            polymer.react();
            polymer.data.len()
        })
        .min()
        .unwrap();
    println!("Shortest: {}", shortest);
}
