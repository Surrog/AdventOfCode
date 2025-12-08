use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    fn from(val: Vec<&str>) -> io::Result<Pos> {
        Ok(Pos {
            x: val[0].parse().map_err(|err| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("failed to parse {} into number: {err}", val[0]),
                )
            })?,
            y: val[1].parse().map_err(|err| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("failed to parse {} into number: {err}", val[1]),
                )
            })?,
            z: val[2].parse().map_err(|err| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("failed to parse {} into number: {err}", val[2]),
                )
            })?,
        })
    }
}

// we only need to compare distances, not the real square root
fn distance(lval: &Pos, rval: &Pos) -> i64 {
    return (lval.x - rval.x) * (lval.x - rval.x)
        + (lval.y - rval.y) * (lval.y - rval.y)
        + (lval.z - rval.z) * (lval.z - rval.z);
}

fn create_circuit(juctions: &Vec<Pos>, connection_limit: usize) -> (usize, i64) {
    let mut distances = Vec::new();

    for lidx in 0..juctions.len() {
        for ridx in (lidx + 1)..juctions.len() {
            distances.push((lidx, ridx, distance(&juctions[lidx], &juctions[ridx])));
        }
    }
    distances.sort_by(|lv, rv| lv.2.cmp(&rv.2));

    let mut distance_to_wall = 0;
    let mut connection_done = 0;
    let mut circuit: Vec<HashSet<usize>> = Vec::new();
    let mut associated: HashSet<usize> = HashSet::new();
    for dis in distances {
        if connection_limit > 0 && connection_done > connection_limit - 1 {
            break;
        }

        let lval_in_circuit = associated.contains(&dis.0);
        let rval_in_circuit = associated.contains(&dis.1);

        if lval_in_circuit || rval_in_circuit {
            let mut lval_idx = None;
            let mut rval_idx = None;
            for idx in 0..circuit.len() {
                if circuit[idx].contains(&dis.0) {
                    lval_idx = Some(idx);
                }
                if circuit[idx].contains(&dis.1) {
                    rval_idx = Some(idx)
                }
                if lval_idx.is_some() && rval_idx.is_some() {
                    break;
                }
            }
            // merge 2 circuit
            if lval_idx.is_some() && rval_idx.is_some() {
                if lval_idx.unwrap() != rval_idx.unwrap() {
                    circuit[lval_idx.unwrap()] = circuit[lval_idx.unwrap()]
                        .union(&circuit[rval_idx.unwrap()])
                        .copied()
                        .collect();
                    circuit.remove(rval_idx.unwrap());
                }
            } else if lval_idx.is_some() {
                circuit[lval_idx.unwrap()].insert(dis.1);
                associated.insert(dis.1);
            } else {
                circuit[rval_idx.unwrap()].insert(dis.0);
                associated.insert(dis.0);
            }
            connection_done += 1;
        } else {
            circuit.push(HashSet::from([dis.0, dis.1]));
            associated.insert(dis.0);
            associated.insert(dis.1);
            connection_done += 1;
        }

        if circuit.len() == 1 && associated.len() == juctions.len() {
            distance_to_wall = juctions[dis.0].x * juctions[dis.1].x;
            break;
        }
    }

    let mut largest_circuit: [usize; 3] = [0, 0, 0];
    for cir in circuit {
        if cir.len() > largest_circuit[0] {
            largest_circuit[2] = largest_circuit[1];
            largest_circuit[1] = largest_circuit[0];
            largest_circuit[0] = cir.len();
        } else if cir.len() > largest_circuit[1] {
            largest_circuit[2] = largest_circuit[1];
            largest_circuit[1] = cir.len();
        } else if cir.len() > largest_circuit[2] {
            largest_circuit[2] = cir.len();
        }
    }

    let res = largest_circuit.iter().product();

    (res, distance_to_wall)
}

#[test]
fn test_distance() {
    let result = distance(&Pos { x: 1, y: 1, z: 1 }, &Pos { x: 2, y: 1, z: 1 });
    assert_eq!(result, 1);

    let result = distance(&Pos { x: 2, y: 1, z: 1 }, &Pos { x: 1, y: 1, z: 1 });
    assert_eq!(result, 1);
}

#[test]
fn test_sort() {
    let mut distances: Vec<(usize, usize, i64)> = vec![(1, 2, 5), (1, 3, 2), (1, 4, 6), (1, 5, 1)];
    distances.sort_by(|lv, rv| lv.2.cmp(&rv.2));
    assert_eq!(distances, vec![(1, 5, 1), (1, 3, 2), (1, 2, 5), (1, 4, 6)])
}

#[test]
fn test_create_circuit() {
    let juctions = vec![
        Pos {
            x: 162,
            y: 817,
            z: 812,
        },
        Pos {
            x: 57,
            y: 618,
            z: 57,
        },
        Pos {
            x: 906,
            y: 360,
            z: 560,
        },
        Pos {
            x: 592,
            y: 479,
            z: 940,
        },
        Pos {
            x: 352,
            y: 342,
            z: 300,
        },
        Pos {
            x: 466,
            y: 668,
            z: 158,
        },
        Pos {
            x: 542,
            y: 29,
            z: 236,
        },
        Pos {
            x: 431,
            y: 825,
            z: 988,
        },
        Pos {
            x: 739,
            y: 650,
            z: 466,
        },
        Pos {
            x: 52,
            y: 470,
            z: 668,
        },
        Pos {
            x: 216,
            y: 146,
            z: 977,
        },
        Pos {
            x: 819,
            y: 987,
            z: 18,
        },
        Pos {
            x: 117,
            y: 168,
            z: 530,
        },
        Pos {
            x: 805,
            y: 96,
            z: 715,
        },
        Pos {
            x: 346,
            y: 949,
            z: 466,
        },
        Pos {
            x: 970,
            y: 615,
            z: 88,
        },
        Pos {
            x: 941,
            y: 993,
            z: 340,
        },
        Pos {
            x: 862,
            y: 61,
            z: 35,
        },
        Pos {
            x: 984,
            y: 92,
            z: 344,
        },
        Pos {
            x: 425,
            y: 690,
            z: 689,
        },
    ];

    let (res, _) = create_circuit(&juctions, 10);
    assert_eq!(res, 40);

    let (_, wall) = create_circuit(&juctions, 0);
    assert_eq!(wall, 25272);
}

fn main() -> io::Result<()> {
    let file = File::open(env::current_dir()?.join("src/input.txt"))?;
    let reader = BufReader::new(file);

    let mut juctions = Vec::new();

    for li in reader.lines() {
        juctions.push(Pos::from(li?.splitn(3, ',').collect())?);
    }

    let (res, wall) = create_circuit(&juctions, 0);
    println!("juction network size: {res}, distance to wall: {wall}");
    Ok(())
}
