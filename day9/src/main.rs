use std::{
    cmp::{max, min},
    env,
    fs::File,
    i64,
    io::{self, BufRead, BufReader}, vec,
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

fn fill_poly(mut mat: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    for li in 0..mat.len() {
        let mut start = 0;
        let mut end = 0;

        for i in 0..mat[li].len() {
            if mat[li][i] == b'X' {
                start = i;
                break;
            }
        }

        for i in 0..mat[li].len() {
            if mat[li][mat[li].len() - i - 1] == b'X' {
                end = mat[li].len() - i;
                break;
            }
        }

        for i in start..end {
            mat[li][i] = b'X';
        }
    }

    mat
}

fn print_mat(mat: &Vec<Vec<u8>>) {
    for l in mat {
        println!("{}", str::from_utf8(&l).unwrap());
    }
}

fn best_rectangle_part2(vertices: &Vec<Position>) -> (Option<(&Position, &Position)>, i64) {
    let vertices_size = vertices.len();
    let mut best_pair = None;
    let mut best_size = 0;

    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    for vert in vertices {
        min_x = min(vert.x, min_x);
        min_y = min(vert.y, min_y);
        max_x = max(vert.x, max_x);
        max_y = max(vert.y, max_y);
    }

    println!("allocate_poly");
    let mut mat = Vec::new();
    let mat_len = (max_y - min_y + 1 + 1) as usize;
    let line_len = (max_x - min_x + 1 + 1) as usize;
    mat.reserve(mat_len);
    for _ in 0..mat_len {
        let mut line = Vec::new();
        line.resize(line_len, b'.');
        mat.push(line);
    }

    println!("draw_poly");
    // fill matrix
    for lidx in 0..vertices_size {
        let x1 = (vertices[lidx].x - min_x) as usize;
        let y1 = (vertices[lidx].y - min_y) as usize;
        let x2 = (vertices[(lidx+1)%vertices_size].x - min_x) as usize;
        let y2 = (vertices[(lidx+1)%vertices_size].y - min_y) as usize;

        if x1 != x2 {
            for x in min(x1, x2)..max(x1, x2) + 1 {
                mat[y1][x] = b'X';                
            } 
        } else if y1 != y2 {
            for y in min(y1, y2)..max(y1, y2) + 1 {
                mat[y][x1] = b'X';                
            } 
        }
    }

    println!("fill_poly");
    mat = fill_poly(mat);
    // print_mat(&mat);

    // try poly
    println!("try polys");
    for lidx in 0..vertices_size {
        println!("{}/{}", lidx, vertices_size);

        for ridx in (lidx+1)..vertices_size {
            let s = size(&vertices[lidx], &vertices[ridx]);
            if s < best_size {
                continue;
            }
            let xs = (min(vertices[lidx].x, vertices[ridx].x));        
            let xe = (max(vertices[lidx].x, vertices[ridx].x));        
            let ys = (min(vertices[lidx].y, vertices[ridx].y));        
            let ye = (max(vertices[lidx].y, vertices[ridx].y));

            let inter = inner_intersect_polygon(&vertices, xs, xe, ys, ye);
            if inter {
                continue;
            }

            let xs = (xs - min_x) as usize ;        
            let xe = (xe - min_x) as usize ;        
            let ys = (ys - min_y) as usize ;        
            let ye = (ye - min_y) as usize ;

            let all_in_polygon = go_outsize(&mat, xs, xe, ys, ye);    
            if all_in_polygon == false {
                continue;
            }
            best_size = s;
            best_pair = Some((&vertices[lidx], &vertices[ridx]));
            println!("found rectangle {} -> {:?}", s, (vertices[lidx], vertices[ridx]));
        }
    }

    // check far idx
    (best_pair, best_size)
}

fn go_outsize(mat: &Vec<Vec<u8>>, xs: usize, xe: usize, ys: usize, ye: usize) -> bool {
    for y in ys..ye {
        for x in xs..xe {
            if mat[y][x] == b'.' {
                return false;
            }
        }
    }
    true
}

fn  inner_intersect_polygon(vertices: &Vec<Position>, xs: i64, xe: i64, ys: i64, ye: i64) -> bool {
    for ver in vertices{
        if ver.x > xs && ver.x < xe && ver.y > ys && ver.y < ye {
            return true;
        }
    }
    false
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

fn print_polygon(vertices: &Vec<Position>, dezoom: i64) {
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    for vert in vertices {
        min_x = min(vert.x, min_x);
        min_y = min(vert.y, min_y);
        max_x = max(vert.x, max_x);
        max_y = max(vert.y, max_y);
    }

    let mut mat = Vec::new();
    let mat_len = (((max_y - min_y) / dezoom) + 1) as usize;
    let line_len = (((max_x - min_x) / dezoom) + 1) as usize;
    mat.reserve(mat_len);
    for _ in 0..mat_len {
        let mut line = Vec::new();
        line.resize(line_len, b'.');
        mat.push(line);
    }

    for vert in vertices {
        let x = ((vert.x - min_x) / dezoom) as usize;
        let y = ((vert.y - min_y) / dezoom) as usize;

        mat[y][x] = b'X';
    }

    for l in mat {
        println!("{}", str::from_utf8(&l).unwrap());
    }
}

fn main() -> io::Result<()> {
    let file = File::open(env::current_dir()?.join("src/input.txt"))?;
    let reader = BufReader::new(file);

    let mut positions: Vec<Position> = Vec::new();

    for l in reader.lines() {
        let pos = parse_line(&l?)?;
        positions.push(pos);
    }

    // print_polygon(&positions, 1000);

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
