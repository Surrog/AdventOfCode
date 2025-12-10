use std::{
    cmp::{max, min},
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

fn parse_line(line: &String) -> io::Result<Position> {
    match line.split_once(',') {
        Some(pair) => Ok(Position {
            x: pair.0.parse().map_err(|err| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("failed to parse input pair: {pair:?}: {err}"),
                )
            })?,
            y: pair.1.parse().map_err(|err| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("failed to parse input pair: {pair:?}: {err}"),
                )
            })?,
        }),
        None => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("failed to parse line {line}"),
        )),
    }
}

fn size(lval: &Position, rval: &Position) -> i64 {
    let x_s = (lval.x - rval.x).abs() + 1;
    let y_s = (lval.y - rval.y).abs() + 1;

    return x_s * y_s;
}

#[test]
fn test_size() {
    assert_eq!(
        size(&Position { x: 7, y: 3 }, &Position { x: 11, y: 1 }),
        15
    );
    assert_eq!(size(&Position { x: 9, y: 7 }, &Position { x: 9, y: 5 }), 3);
    assert_eq!(size(&Position { x: 9, y: 5 }, &Position { x: 2, y: 3 }), 24);
    assert_eq!(
        size(&Position { x: 2, y: 5 }, &Position { x: 11, y: 1 }),
        50
    );
    assert_eq!(size(&Position { x: 7, y: 3 }, &Position { x: 2, y: 3 }), 6);
    assert_eq!(
        size(&Position { x: 7, y: 1 }, &Position { x: 11, y: 7 }),
        35
    );
}

fn best_rectangle(positions: &Vec<Position>) -> (Option<(Position, Position)>, i64) {
    let mut best_pair = None;
    let mut best_size = 0;

    for l_idx in 0..positions.len() {
        for r_idx in l_idx + 1..positions.len() {
            let size = size(&positions[l_idx], &positions[r_idx]);
            if size > best_size {
                best_size = size;
                best_pair = Some((positions[l_idx], positions[r_idx]))
            }
        }
    }
    (best_pair, best_size)
}

#[test]
fn test_best_rectangle() {
    let input = vec![
        Position { x: 7, y: 1 },
        Position { x: 11, y: 1 },
        Position { x: 11, y: 7 },
        Position { x: 9, y: 7 },
        Position { x: 9, y: 5 },
        Position { x: 2, y: 5 },
        Position { x: 2, y: 3 },
        Position { x: 7, y: 3 },
    ];

    let (index, size) = best_rectangle(&input);
    assert!(index.is_some());
    assert_eq!(size, 50);
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
    None,
}

fn direction(lval: &Position, rval: &Position) -> Direction {
    if lval.x == rval.x && lval.y < rval.y {
        return Direction::Down;
    }
    if lval.x == rval.x && lval.y > rval.y {
        return Direction::Up;
    }
    if lval.x < rval.x && lval.y == rval.y {
        return Direction::Right;
    }
    if lval.x > rval.x && lval.y == rval.y {
        return Direction::Left;
    }
    Direction::None
}

struct Line {
    start: Position,
    end: Position,
}

fn line_intersect(lval: &Line, rval: &Line) -> Option<Position> {
    let x1 = lval.start.x;
    let x2 = lval.end.x;
    let x3 = rval.start.x;
    let x4 = rval.end.x;

    let y1 = lval.start.y;
    let y2 = lval.end.y;
    let y3 = rval.start.y;
    let y4 = rval.end.y;

    if lval.start == rval.start {
        return Some(lval.start);
    }
    if lval.start == rval.end {
        return Some(lval.start);
    }
    if lval.end == rval.start {
        return Some(lval.end);
    }
    if lval.end == rval.end {
        return Some(lval.end);
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) as f32
        / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4)) as f32;
    let u = ((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) as f32
        / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4)) as f32;

    if t >= 0. && t <= 1. && u >= 0. && u <= 1. {
        let x = x1 as f32 + t * (x2 - x1) as f32;
        let y = y1 as f32 + t * (y2 - y1) as f32;
        return Some(Position {
            x: x.round() as i64,
            y: y.round() as i64,
        });
    }
    None
}

#[test]
fn test_line_intersect() {
    // ..2...
    // .1....
    // .32...
    // ......
    // ......
    // .....1
    // .....3

    let l1 = Line {
        start: Position { x: 1, y: 1 },
        end: Position { x: 5, y: 5 },
    };

    let l2 = Line {
        start: Position { x: 2, y: 2 },
        end: Position { x: 2, y: 0 },
    };

    let inter = line_intersect(&l1, &l2);
    assert!(inter.is_some());
    assert_eq!(inter.unwrap(), Position { x: 2, y: 2 });

    let l3 = Line {
        start: Position { x: 2, y: 1 },
        end: Position { x: 6, y: 5 },
    };

    let inter = line_intersect(&l1, &l3);
    assert!(inter.is_none());

    let l4 = Line {
        start: Position { x: 2, y: 2 },
        end: Position { x: 5, y: 5 },
    };

    let inter = line_intersect(&l1, &l4);
    assert!(inter.is_some());

    let l5 = Line {
        start: Position { x: 2, y: 1 },
        end: Position { x: 5, y: 5 },
    };

    let inter = line_intersect(&l1, &l4);
    assert!(inter.is_some());
    assert_eq!(inter.unwrap(), Position { x: 5, y: 5 });
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PolygonClockwise {
    Clock,
    Anti,
}

fn check_coord_direction(vertices: &Vec<Position>) -> PolygonClockwise {
    let mut sum_over_the_edge = 0;
    for i in 0..vertices.len() {
        sum_over_the_edge += (vertices[(i + 1) % vertices.len()].x - vertices[i].x)
            * (vertices[(i + 1) % vertices.len()].y + vertices[i].y)
    }

    if sum_over_the_edge > 0 {
        return PolygonClockwise::Anti;
    }
    PolygonClockwise::Clock
}

fn get_bad_direction(vertices: &Vec<Position>) -> Vec<(Direction, Direction)> {
    match check_coord_direction(vertices) {
        PolygonClockwise::Clock => vec![
            (Direction::Up, Direction::Left),
            (Direction::Right, Direction::Up),
            (Direction::Down, Direction::Right),
            (Direction::Left, Direction::Down),
        ],
        PolygonClockwise::Anti => vec![
            (Direction::Left, Direction::Down),
            (Direction::Down, Direction::Left),
            (Direction::Left, Direction::Up),
            (Direction::Up, Direction::Right),
        ],
    }
}

#[test]
fn test_coord_direction() {
    let input = vec![
        Position { x: 7, y: 1 },
        Position { x: 11, y: 1 },
        Position { x: 11, y: 7 },
        Position { x: 9, y: 7 },
        Position { x: 9, y: 5 },
        Position { x: 2, y: 5 },
        Position { x: 2, y: 3 },
        Position { x: 7, y: 3 },
    ];

    let clock = check_coord_direction(&input);
    assert_eq!(clock, PolygonClockwise::Clock);

    let reverse: Vec<Position> = input.iter().rev().copied().collect();
    let clock = check_coord_direction(&reverse);
    assert_eq!(clock, PolygonClockwise::Anti);
}

fn best_rectangle_part2(vertices: &Vec<Position>) -> (Option<(&Position, &Position)>, i64) {
    let vertices_size = vertices.len();
    let mut best_pair = None;
    let mut best_size = 0;

    let outside_dirs = get_bad_direction(vertices);

    for l_idx in 0..vertices_size {
        let current = &vertices[l_idx];
        let second = &vertices[(l_idx + 1) % vertices_size];
        let third = &vertices[(l_idx + 2) % vertices_size];
        let fourth = Position {
            x: current.x,
            y: third.y,
        };

        let s = size(current, second);
        if s > best_size {
            best_size = s;
            best_pair = Some((current, second));
        }

        let s = size(current, third);
        if s <= best_size {
            continue;
        }

        println!(
            "considering {:?}, {:?}, {:?}, {:?}",
            current, second, third, fourth
        );

        let mut bad_dirs = false;
        let current_dir = direction(current, second);
        let second_dir = direction(second, third);
        for outdir in &outside_dirs {
            if current_dir == outdir.0 && second_dir == outdir.1 {
                bad_dirs = true;
            }
        }
        if bad_dirs {
            continue; // we are doing a rectange outise of the polygon
        }

        let mut rectangle_limit = (i64::MAX, 0, i64::MAX, 0);
        for i in 0..3 {
            rectangle_limit.0 = min(rectangle_limit.0, vertices[(l_idx + i) % vertices_size].x);
            rectangle_limit.1 = max(rectangle_limit.1, vertices[(l_idx + i) % vertices_size].x);
            rectangle_limit.2 = min(rectangle_limit.2, vertices[(l_idx + i) % vertices_size].y);
            rectangle_limit.3 = max(rectangle_limit.3, vertices[(l_idx + i) % vertices_size].y);
        }

        let mut vectice_stricly_inside = false;
        for pos in vertices {
            if pos.x > rectangle_limit.0
                && pos.x < rectangle_limit.1
                && pos.y > rectangle_limit.2
                && pos.y < rectangle_limit.3
            {
                vectice_stricly_inside = true;
            }
        }
        if vectice_stricly_inside {
            // polygon too concave to do this rectangle
            continue;
        }

        best_size = s;
        best_pair = Some((current, third));
    }
    (best_pair, best_size)
}

#[test]
fn test_best_rectangle_part2() {
    let input = vec![
        Position { x: 7, y: 1 },
        Position { x: 11, y: 1 },
        Position { x: 11, y: 7 },
        Position { x: 9, y: 7 },
        Position { x: 9, y: 5 },
        Position { x: 2, y: 5 },
        Position { x: 2, y: 3 },
        Position { x: 7, y: 3 },
    ];

    let (index, size) = best_rectangle_part2(&input);
    assert!(index.is_some());
    assert_eq!(size, 24);
    let (l, r) = index.unwrap();
    println!("pos {:?}, {:?}", l, r);
}

fn main() -> io::Result<()> {
    let file = File::open(env::current_dir()?.join("src/input.txt"))?;
    let reader = BufReader::new(file);

    let mut positions: Vec<Position> = Vec::new();

    for l in reader.lines() {
        let pos = parse_line(&l?)?;
        positions.push(pos);
    }

    let (best_pair, best_size) = best_rectangle_part2(&positions);

    match best_pair {
        Some(pair) => println!("the best size: {best_size}, pair: {pair:?}"),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "failed to find the best pair",
            ));
        }
    };

    Ok(())
}
