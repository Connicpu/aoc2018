static INPUT: &str = include_str!("day08.txt");

fn input() -> impl Iterator<Item = i32> {
    INPUT.split_whitespace().filter_map(|s| s.parse().ok())
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>,
}

fn parse_node(iter: &mut impl Iterator<Item = i32>) -> Option<Node> {
    let child_count = iter.next()?;
    let meta_count = iter.next()?;

    let mut children = Vec::with_capacity(child_count as usize);
    for _ in 0..child_count {
        children.push(parse_node(iter)?);
    }

    let mut metadata = Vec::with_capacity(meta_count as usize);
    for _ in 0..meta_count {
        metadata.push(iter.next()?);
    }

    Some(Node { children, metadata })
}

impl Node {
    fn self_sum(&self) -> i32 {
        self.metadata.iter().cloned().sum()
    }

    fn child_sum(&self) -> i32 {
        self.children.iter().map(|c| c.meta_sum()).sum()
    }

    fn meta_sum(&self) -> i32 {
        self.self_sum() + self.child_sum()
    }
    fn indexed_sum(&self) -> i32 {
        self.metadata
            .iter()
            .filter_map(|&i| if i == 0 { None } else { Some((i - 1) as usize) })
            .filter_map(move |i| self.children.get(i))
            .map(|c| c.value())
            .sum()
    }

    fn value(&self) -> i32 {
        if self.children.is_empty() {
            self.self_sum()
        } else {
            self.indexed_sum()
        }
    }
}

fn main() {
    let tree = parse_node(&mut input()).unwrap();
    println!("Metadata sum: {}", tree.meta_sum());
    println!("Tree value: {}", tree.value());
}
