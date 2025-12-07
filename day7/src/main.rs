use std::{
    env, fs::File, io::{self, BufRead, BufReader}
};

#[allow(dead_code)]
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

fn propagate_quantum_tachyon_timeline(matrix: &Vec<Vec<u8>>, timelines: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut next_timeline =  Vec::new();
    next_timeline.reserve(timelines.len() * 2);

    for cur_timeline in timelines {
        if matrix[cur_timeline.0][cur_timeline.1] == b'.' {
            let key = (cur_timeline.0 + 1, cur_timeline.1);
            next_timeline.push(key);
        }
        if matrix[cur_timeline.0][cur_timeline.1] == b'^' {
            if cur_timeline.1 > 0 {
                let key = (cur_timeline.0 + 1 ,cur_timeline.1 - 1);
                next_timeline.push(key);
            }
            if cur_timeline.1 < matrix[cur_timeline.0].len() - 1 {
                let key =(cur_timeline.0 + 1 ,cur_timeline.1 + 1);
                next_timeline.push(key);
            }
        }
    }
    next_timeline
}

fn propagate_quantum_tachyon(matrix: &Vec<Vec<u8>>, line: usize, col: usize) -> usize {
    let mut timelines = Vec::from([(line, col)]);

    for line in line..matrix.len() {
        println!("line: {line}/{}: timelines: {}", matrix.len(), timelines.len());
        timelines = propagate_quantum_tachyon_timeline(matrix, &timelines);
    }
    return timelines.len();
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

    let mut s_line = 0;
    let mut s_col = 0;
    for line in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[line][col] == b'S' {
                s_line = line + 1;
                s_col = col;
            }
        }
    }

    let timeline = propagate_quantum_tachyon(&matrix, s_line, s_col);
    assert_eq!(timeline, 40);
}



fn main() -> io::Result<()> {
    let file = File::open(env::current_dir()?.join("src/input.txt"))?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        matrix.push(line?.as_bytes().to_vec());
    }

    let mut s_line = 0;
    let mut s_col = 0;
    for line in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[line][col] == b'S' {
                s_line = line + 1;
                s_col = col;
            }
        }
    }

    
    let timeline = propagate_quantum_tachyon(&matrix, s_line, s_col); 
    print!("total timeline: {timeline}");

    Ok(())
}
