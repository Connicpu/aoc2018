#![feature(duration_as_u128)]

use self::Direction::*;
use math2d::*;

use direct2d::{
    brush::{Brush, SolidColorBrush},
    device_context::DeviceContext,
    enums::BitmapOptions,
    geometry::PathGeometry,
    image::{Bitmap, Bitmap1},
    stroke_style::StrokeStyle,
    RenderTarget,
};

struct Grid {
    grid: [[u8; 150]; 150],
    carts: Vec<Cart>,
    crashes: Vec<Point2i>,
}

#[derive(Copy, Clone)]
struct Cart {
    pos: Point2i,
    dir: Direction,
    turn: usize,
}

impl Cart {
    fn forward(&mut self) {
        let p = self.pos;
        self.pos = match self.dir {
            North => (p.x, p.y - 1).into(),
            South => (p.x, p.y + 1).into(),
            East => (p.x + 1, p.y).into(),
            West => (p.x - 1, p.y).into(),
        };
    }
}

fn parse_grid() -> Grid {
    let mut grid = [[b' '; 150]; 150];
    let mut carts = vec![];

    for (y, line) in INPUT.lines().enumerate() {
        if line.trim() == "" {
            continue;
        }

        for (x, &c) in line.as_bytes().iter().enumerate() {
            match c {
                b'|' | b'-' | b'/' | b'\\' | b'+' => grid[y][x] = c,
                b'^' | b'v' | b'<' | b'>' => {
                    let pos = (x as i32, y as i32).into();
                    let dir = Direction::for_cart(c);
                    let turn = 0;

                    carts.push(Cart { pos, dir, turn });
                    grid[y][x] = dir.straight_track();
                }
                _ => (),
            }
        }
    }

    carts.reverse();

    let crashes = Vec::with_capacity(carts.len() - 1);
    Grid {
        grid,
        carts,
        crashes,
    }
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn for_cart(c: u8) -> Direction {
        match c {
            b'^' => North,
            b'v' => South,
            b'<' => West,
            b'>' => East,
            _ => unreachable!(),
        }
    }

    fn straight_track(self) -> u8 {
        match self {
            North | South => b'|',
            East | West => b'-',
        }
    }
}

fn directions(c: u8, from: Direction) -> &'static [Direction] {
    match (c, from) {
        (b'|', North) => &[North],
        (b'|', South) => &[South],
        (b'|', East) => &[],
        (b'|', West) => &[],

        (b'-', North) => &[],
        (b'-', South) => &[],
        (b'-', East) => &[East],
        (b'-', West) => &[West],

        (b'/', North) => &[East],
        (b'/', South) => &[West],
        (b'/', East) => &[North],
        (b'/', West) => &[South],

        (b'\\', North) => &[West],
        (b'\\', South) => &[East],
        (b'\\', East) => &[South],
        (b'\\', West) => &[North],

        (b'+', North) => &[West, North, East],
        (b'+', South) => &[East, South, West],
        (b'+', East) => &[North, East, South],
        (b'+', West) => &[South, West, North],

        _ => unreachable!(),
    }
}

fn tick(grid: &mut Grid) {
    let mut crashes = Vec::new();
    for i in 0..grid.carts.len() {
        if crashes.contains(&i) {
            continue;
        }

        let cart = grid.carts[i];
        let track = grid.grid[cart.pos.y as usize][cart.pos.x as usize];

        let directions = directions(track, cart.dir);
        match directions.len() {
            1 => {
                grid.carts[i].dir = directions[0];
            }
            3 => {
                grid.carts[i].dir = directions[cart.turn % 3];
                grid.carts[i].turn += 1;
            }
            _ => unreachable!(),
        }

        grid.carts[i].forward();

        let cart = grid.carts[i];
        for j in 0..grid.carts.len() {
            if i != j && cart.pos == grid.carts[j].pos {
                grid.crashes.push(cart.pos);
                crashes.push(i);
                crashes.push(j);
            }
        }
    }

    crashes.sort_by_key(|i| std::usize::MAX - i);
    for crashed in crashes {
        grid.carts.remove(crashed);
    }
}

fn part1() {
    let mut grid = parse_grid();

    let first_crash = loop {
        tick(&mut grid);

        if !grid.crashes.is_empty() {
            break grid.crashes[0];
        }
    };

    println!("{:?}", first_crash);
}

fn part2() {
    let mut grid = parse_grid();

    let last_cart = loop {
        tick(&mut grid);

        if grid.carts.len() == 1 {
            break grid.carts[0].pos;
        } else if grid.carts.is_empty() {
            panic!();
        }
    };

    println!("{:?}", last_cart);
}

fn draw_track(
    track: u8,
    left: u8,
    right: u8,
    rt: &mut RenderTarget,
    brush: &Brush,
    stroke: Option<&StrokeStyle>,
    arc: &PathGeometry,
    x: f32,
    y: f32,
) {
    use math2d::Matrix3x2f;
    let x = x + 0.5;
    let y = y + 0.5;
    match track {
        b'-' => {
            rt.draw_line((x + 0.0, y + 0.5), (x + 1.0, y + 0.5), brush, 0.2, stroke);
        }
        b'|' => {
            rt.draw_line((x + 0.5, y + 0.0), (x + 0.5, y + 1.0), brush, 0.2, stroke);
        }
        b'/' => {
            if ![b' ', b'|'].contains(&left) {
                let transform =
                    Matrix3x2f::scaling([1.0, 1.0], (0.5, 0.5)) * Matrix3x2f::translation([x, y]);
                rt.set_transform(&transform);
                rt.draw_geometry(arc, brush, 0.2, stroke);
                rt.set_transform(&Matrix3x2f::IDENTITY);
            }
            if ![b' ', b'|'].contains(&right) {
                let transform =
                    Matrix3x2f::scaling([-1.0, -1.0], (0.5, 0.5)) * Matrix3x2f::translation([x, y]);
                rt.set_transform(&transform);
                rt.draw_geometry(arc, brush, 0.2, stroke);
                rt.set_transform(&Matrix3x2f::IDENTITY);
            }
        }
        b'\\' => {
            if ![b' ', b'|'].contains(&left) {
                let transform =
                    Matrix3x2f::scaling([1.0, -1.0], (0.5, 0.5)) * Matrix3x2f::translation([x, y]);
                rt.set_transform(&transform);
                rt.draw_geometry(arc, brush, 0.2, stroke);
                rt.set_transform(&Matrix3x2f::IDENTITY);
            }
            if ![b' ', b'|'].contains(&right) {
                let transform =
                    Matrix3x2f::scaling([-1.0, 1.0], (0.5, 0.5)) * Matrix3x2f::translation([x, y]);
                rt.set_transform(&transform);
                rt.draw_geometry(arc, brush, 0.2, stroke);
                rt.set_transform(&Matrix3x2f::IDENTITY);
            }
        }
        b'+' => {
            rt.draw_line((x + 0.0, y + 0.5), (x + 1.0, y + 0.5), brush, 0.2, stroke);
            rt.draw_line((x + 0.5, y + 0.0), (x + 0.5, y + 1.0), brush, 0.2, stroke);
        }
        _ => {}
    }
}

fn draw_cart(cart: &Cart, rt: &mut RenderTarget, brush: &Brush) {
    let size = rt.size();
    let (ox, oy) = if size.width > size.height {
        ((size.width - size.height) / 2.0, 0.0)
    } else {
        (0.0, (size.height - size.width) / 2.0)
    };

    let x = ox + cart.pos.x as f32 + 0.5;
    let y = oy + cart.pos.y as f32 + 0.5;

    rt.fill_ellipse(((x + 0.5, y + 0.5), 0.6, 0.6), brush);
    match cart.dir {
        North => rt.fill_ellipse(((x + 0.5, y + 0.25), 0.3, 0.6), brush),
        South => rt.fill_ellipse(((x + 0.5, y + 0.75), 0.3, 0.6), brush),
        East => rt.fill_ellipse(((x + 0.75, y + 0.5), 0.6, 0.3), brush),
        West => rt.fill_ellipse(((x + 0.25, y + 0.5), 0.6, 0.3), brush),
    }
}

fn draw_crash(crash: &Point2i, rt: &mut RenderTarget, brush: &Brush, stroke: Option<&StrokeStyle>) {
    let size = rt.size();
    let (ox, oy) = if size.width > size.height {
        ((size.width - size.height) / 2.0, 0.0)
    } else {
        (0.0, (size.height - size.width) / 2.0)
    };

    let x = ox + crash.x as f32 + 0.5;
    let y = oy + crash.y as f32 + 0.5;
    let crash = Point2f::new(x, y);

    rt.draw_line(crash + [0.0, 0.0], crash + [1.0, 1.0], brush, 0.2, stroke);
    rt.draw_line(crash + [1.0, 0.0], crash + [0.0, 1.0], brush, 0.2, stroke);
}

fn draw_tex(tex: &Bitmap, rt: &mut RenderTarget) {
    use direct2d::enums::BitmapInterpolationMode::Linear;
    let size = rt.size();
    let (size, x, y) = if size.width > size.height {
        (size.height, (size.width - size.height) / 2.0, 0.0)
    } else {
        (size.width, 0.0, (size.height - size.width) / 2.0)
    };

    rt.draw_bitmap(
        tex,
        [x, y, x + size, y + size],
        1.0,
        Linear,
        [0.0, 0.0, GRIDSIZEF, GRIDSIZEF],
    );
}

fn fix_dpi(rt: &mut direct2d::RenderTarget) {
    let psize = rt.pixel_size();
    let scale = if psize.width > psize.height {
        psize.height as f32 / GRIDSIZEF * 96.0
    } else {
        psize.width as f32 / GRIDSIZEF * 96.0
    };
    rt.set_dpi(scale, scale);
}

fn draw_map(grid: &Grid, rt: &mut RenderTarget) {
    use direct2d::enums::CapStyle;
    use direct2d::enums::{FigureBegin::Hollow, FigureEnd::Open};
    use math2d::{ArcSegment, ArcSize, SweepDirection};

    let track_brush = SolidColorBrush::new(rt, TRACK_COLOR).unwrap();
    let stroke = StrokeStyle::create(&rt.factory())
        .with_start_cap(CapStyle::Round)
        .with_end_cap(CapStyle::Round)
        .build()
        .unwrap();

    let arc = PathGeometry::create(&rt.factory())
        .unwrap()
        .with_figure((0.0, 0.5), Hollow, Open, |figure| {
            figure.add_arc(&ArcSegment {
                point: (0.5, 0.0).into(),
                size: (0.3, 0.3).into(),
                rotation_angle: 0.0,
                sweep_direction: SweepDirection::CounterClockwise,
                arc_size: ArcSize::Large,
            })
        })
        .finish()
        .unwrap();

    rt.begin_draw();
    rt.clear(BGCOLOR);

    for y in 0..GRIDSIZE {
        for x in 0..GRIDSIZE {
            const L: usize = 0;
            const R: usize = GRIDSIZE - 1;
            let (left, right) = match x {
                L => (b' ', grid.grid[y][x + 1]),
                R => (grid.grid[y][x - 1], b' '),
                _ => (grid.grid[y][x - 1], grid.grid[y][x + 1]),
            };
            draw_track(
                grid.grid[y][x],
                left,
                right,
                rt,
                &track_brush,
                Some(&stroke),
                &arc,
                x as f32,
                y as f32,
            );
        }
    }

    rt.end_draw().unwrap();
}

fn draw_carts(grid: &Grid, rt: &mut RenderTarget) {
    use direct2d::enums::CapStyle;

    let cart_brush = SolidColorBrush::new(rt, CART_COLOR).unwrap();
    let stroke = StrokeStyle::create(&rt.factory())
        .with_start_cap(CapStyle::Round)
        .with_end_cap(CapStyle::Round)
        .build()
        .unwrap();

    for cart in grid.carts.iter() {
        draw_cart(cart, rt, &cart_brush);
    }

    for crash in grid.crashes.iter() {
        draw_crash(crash, rt, &cart_brush, Some(&stroke));
    }
}

fn viz() {
    use direct2d::factory::Factory1;
    use direct3d11::enums::{BindFlags, CreateDeviceFlags};
    use direct3d11::Texture2D;
    use dxgi::enums::{Format, PresentFlags};
    use dxgi::swap_chain::SwapChain1;
    use std::time::{Duration, Instant};
    use winit::{os::windows::WindowExt, Event, EventsLoop, VirtualKeyCode, Window, WindowEvent};

    let mut evloop = EventsLoop::new();
    let window = Window::new(&evloop).unwrap();
    window.set_title("Day 13");

    let dxgi: dxgi::factory::Factory2 = dxgi::factory::create().unwrap();

    let (_, d3d, _) = direct3d11::Device::create()
        .with_flags(CreateDeviceFlags::BGRA_SUPPORT | CreateDeviceFlags::DEBUG)
        .build()
        .unwrap();

    let mut chain = SwapChain1::create_hwnd(&dxgi, &d3d.as_dxgi())
        .with_hwnd(window.get_hwnd() as _)
        .build()
        .unwrap();

    let d2d = Factory1::new().unwrap();
    let dev = direct2d::Device::create(&d2d, &d3d.as_dxgi()).unwrap();
    let mut ctx = DeviceContext::create(&dev).unwrap();

    // Create a texture to render to
    let tex = direct3d11::texture2d::Texture2D::create(&d3d)
        .with_size(4096, 4096)
        .with_format(Format::R8G8B8A8Unorm)
        .with_bind_flags(BindFlags::RENDER_TARGET | BindFlags::SHADER_RESOURCE)
        .build()
        .unwrap();

    let map = Bitmap1::create(&ctx)
        .with_dxgi_surface(&tex.as_dxgi())
        .with_dpi(96.0 * 4096.0 / GRIDSIZEF, 96.0 * 4096.0 / GRIDSIZEF)
        .with_options(BitmapOptions::TARGET)
        .build()
        .unwrap();

    let backbuffer: Texture2D = chain.buffer(0).unwrap();
    let mut target = Some(
        Bitmap1::create(&ctx)
            .with_dxgi_surface(&backbuffer.as_dxgi())
            .with_options(BitmapOptions::TARGET | BitmapOptions::CANNOT_DRAW)
            .build()
            .unwrap(),
    );
    drop(backbuffer);

    let mut grid = parse_grid();

    ctx.set_target(&map);
    fix_dpi(&mut ctx);
    draw_map(&grid, &mut ctx);

    ctx.set_target(target.as_ref().unwrap());
    ctx.begin_draw();
    fix_dpi(&mut ctx);
    draw_tex(&map, &mut ctx);
    draw_carts(&grid, &mut ctx);
    ctx.end_draw().unwrap();
    chain.present(0, PresentFlags::NONE).unwrap();

    let mut quit = false;
    let mut next_tick = Instant::now() + Duration::from_millis(1_500);
    let mut step = Duration::from_millis(0_500);
    while !quit {
        let now = Instant::now();

        evloop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                quit = true;
            }

            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                target = None;
                ctx.set_target(&map);

                let (width, height) = size.to_physical(window.get_hidpi_factor()).into();
                chain
                    .resize_buffers()
                    .dimensions(width, height)
                    .finish()
                    .unwrap();

                let backbuffer: Texture2D = chain.buffer(0).unwrap();
                target = Some(
                    Bitmap1::create(&ctx)
                        .with_dxgi_surface(&backbuffer.as_dxgi())
                        .with_options(BitmapOptions::TARGET | BitmapOptions::CANNOT_DRAW)
                        .build()
                        .unwrap(),
                );
                drop(backbuffer);
                ctx.set_target(target.as_ref().unwrap());
            }

            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input: key, .. },
                ..
            } => {
                if key.virtual_keycode == Some(VirtualKeyCode::R) {
                    grid = parse_grid();
                } else if key.virtual_keycode == Some(VirtualKeyCode::W) {
                    if step.as_millis() > 100 {
                        step -= Duration::from_millis(50);
                    } else if step.as_millis() > 10 {
                        step -= Duration::from_millis(10);
                    }
                } else if key.virtual_keycode == Some(VirtualKeyCode::S) {
                    if step.as_millis() < 100 {
                        step += Duration::from_millis(10);
                    } else {
                        step += Duration::from_millis(50);
                    }
                } else if key.virtual_keycode == Some(VirtualKeyCode::Z) {
                    step = Duration::from_millis(0_500);
                }
                next_tick = now + step;

                ctx.begin_draw();
                fix_dpi(&mut ctx);
                draw_tex(&map, &mut ctx);
                draw_carts(&grid, &mut ctx);
                chain.present(0, PresentFlags::NONE).unwrap();
                ctx.end_draw().unwrap();
                next_tick = now + step;
            }

            _ => (),
        });

        if now >= next_tick {
            tick(&mut grid);

            fix_dpi(&mut ctx);
            ctx.begin_draw();
            ctx.clear(BGCOLOR);
            draw_tex(&map, &mut ctx);
            draw_carts(&grid, &mut ctx);
            ctx.end_draw().unwrap();
            chain.present(0, PresentFlags::NONE).unwrap();

            next_tick += step;
        }

        if now > next_tick {
            std::thread::sleep(std::cmp::min(Duration::from_millis(16), now - next_tick));
        }
    }
}

static INPUT: &str = include_str!("day13.txt");

const GRIDSIZE: usize = 150;
const GRIDSIZEF: f32 = (GRIDSIZE + 1) as f32;
const BGCOLOR: u32 = 0xFF_FF_FF;
const TRACK_COLOR: u32 = 0x77_77_77;
const CART_COLOR: u32 = 0xFF_00_7F;

fn main() {
    if false {
        part1();
        part2();
    } else {
        viz();
    }
}
