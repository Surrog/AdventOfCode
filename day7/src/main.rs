use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn propagate_tachyon(mut matrix: Vec<Vec<u8>>) -> (i32, Vec<Vec<u8>>) {
    let mut split_count = 0; 

    for line in 1..matrix.len() {
        for col in 0..matrix[line].len() {
            if matrix[line-1][col] == b'S' || matrix[line-1][col] == b'|' {
                if matrix[line][col] == b'.' {
                    matrix[line][col] = b'|';
                }
            }
        }
        for col in 0..matrix[line].len() {
            if matrix[line-1][col] == b'S' || matrix[line-1][col] == b'|' {
                if matrix[line][col] == b'^' {
                    split_count += 1;
                    if col > 0 && matrix[line][col-1] == b'.' {
                        matrix[line][col-1] = b'|';
                    }
                    if col < matrix[line].len() - 1 && matrix[line][col+1] == b'.' {
                        matrix[line][col+1] = b'|';
                    }
                }
            }
        }
    }
    (split_count, matrix)
}

fn propagate_quantum_tachyon(mut matrix: Vec<Vec<u8>>, start_line: usize, start_col: usize) -> i32 {
    //dump_mat(&matrix);

    for line in start_line..matrix.len() {
        for col in start_col..matrix[line].len() {
            if matrix[line-1][col] == b'S' || matrix[line-1][col] == b'|' {
                if matrix[line][col] == b'.' {
                    matrix[line][col] = b'|';
                }
            }
        }
        for col in start_col..matrix[line].len() {
            if matrix[line-1][col] == b'S' || matrix[line-1][col] == b'|' {
                if matrix[line][col] == b'^' {
                    let mut timeline = 0;
                    if col > 0 && matrix[line][col-1] == b'.' {
                        matrix[line][col-1] = b'|';
                        timeline += propagate_quantum_tachyon(matrix.clone(), line + 1, col-1);
                    }
                    if col < matrix[line].len() - 1 && matrix[line][col+1] == b'.' {
                        matrix[line][col+1] = b'|';
                        timeline += propagate_quantum_tachyon(matrix, line + 1, col+1);
                    }
                    return timeline
                }
            }
        }
    }
    1
}

fn dump_mat(mat: &Vec<Vec<u8>>) {
    for line in mat {
        println!("{}",str::from_utf8(&line).unwrap());
    }
}

#[test]
fn test_propagate_tachyon() {
    let matrix =[
".......S.......".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
".......^.......".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
"......^.^......".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
".....^.^.^.....".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
"....^.^...^....".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
"...^.^...^.^...".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
"..^...^.....^..".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
".^.^.^.^.^...^.".as_bytes().to_vec(),
"...............".as_bytes().to_vec()].to_vec();

    let (split_count, _) = propagate_tachyon(matrix);
    assert_eq!(split_count, 21);
}

#[test]
fn test_propagate_quantum_tachyon() {
    let matrix =[
".......S.......".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
".......^.......".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
"......^.^......".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
".....^.^.^.....".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
"....^.^...^....".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
"...^.^...^.^...".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
"..^...^.....^..".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
".^.^.^.^.^...^.".as_bytes().to_vec(),
"...............".as_bytes().to_vec()].to_vec();

    let timeline = propagate_quantum_tachyon(matrix, 1, 0);
    assert_eq!(timeline, 40);
}



fn main() -> io::Result<()> {
    let file = File::open(env::current_dir()?.join("src/input.txt"))?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        matrix.push(line?.as_bytes().to_vec());
    }
    
    let timeline = propagate_quantum_tachyon(matrix, 1, 0); 
    print!("total timeline: {timeline}");

    Ok(())
}
