use aoc2018::{extract_columns, parse_columns};
use math2d::{Point2i, Recti, Vector2i};

struct Light {
    pos: Point2i,
    vel: Vector2i,
}

static INPUT: &str = include_str!("day10.txt");

fn lights() -> impl Iterator<Item = Light> {
    INPUT
        .split('\n')
        .map(|line| parse_columns(line, |c| !char::is_numeric(c) && c != '-'))
        .filter_map(extract_columns![(x, x, x, x)])
        .map(|(px, py, vx, vy)| ((px, py).into(), [vx, vy].into()))
        .map(|(pos, vel)| Light { pos, vel })
}

struct Sky {
    lights: Vec<Light>,
    ticks: usize,
}

impl Sky {
    fn tick(&mut self) {
        self.ticks += 1;
        for light in self.lights.iter_mut() {
            light.pos = light.pos + light.vel;
        }
    }

    fn untick(&mut self) {
        self.ticks -= 1;
        for light in self.lights.iter_mut() {
            light.pos = light.pos - light.vel;
        }
    }

    fn bounds(&self) -> Recti {
        self.lights
            .iter()
            .fold(Recti::EMPTY, |r, p| r.combined_with(p.pos))
    }

    fn light_at(&self, x: i32, y: i32) -> bool {
        for light in self.lights.iter() {
            if light.pos.x == x && light.pos.y == y {
                return true;
            }
        }
        false
    }

    fn print(&self) {
        let bounds = self.bounds();
        println!("+{:-<1$}+", "", (bounds.right - bounds.left + 3) as usize);

        let bounds = self.bounds();
        for y in bounds.top..=bounds.bottom {
            print!("| ");
            for x in bounds.left..=bounds.right {
                if self.light_at(x, y) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!(" |");
        }

        println!("+{:-<1$}+", "", (bounds.right - bounds.left + 3) as usize);
        println!("time: {} seconds", self.ticks);
    }
}

fn main() {
    let mut sky = Sky {
        lights: lights().collect(),
        ticks: 0,
    };

    let mut prev_area = std::i64::MAX;
    loop {
        let bounds = sky.bounds();
        let area = bounds.area();

        if area > prev_area {
            sky.untick();
            sky.print();
            break;
        }

        prev_area = area;
        sky.tick();
    }
}
