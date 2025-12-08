use std::{
    collections::HashMap, env, fs::File, io::{self, BufRead, BufReader}
};

fn propagate_quantum_tachyon_timeline(matrix: &Vec<Vec<u8>>, timelines: &HashMap<(usize, usize), usize>) -> HashMap<(usize, usize), usize> {
    let mut next_timeline =  HashMap::new();
    next_timeline.reserve(timelines.len() * 2);

    for (cur_timeline, val) in timelines {
        if matrix[cur_timeline.0][cur_timeline.1] == b'^' {
            if cur_timeline.1 > 0 {
                let key = (cur_timeline.0 + 1 ,cur_timeline.1 - 1);
                let new_val = next_timeline.get(&key).or(Some(&0)).unwrap()+val;
                next_timeline.insert(key, new_val);
            }
            if cur_timeline.1 < matrix[cur_timeline.0].len() - 1 {
                let key =(cur_timeline.0 + 1 ,cur_timeline.1 + 1);
                let new_val = next_timeline.get(&key).or(Some(&0)).unwrap()+val;
                next_timeline.insert(key, new_val);
            }
        } else {
            let key = (cur_timeline.0 + 1, cur_timeline.1);
            let new_val = next_timeline.get(&key).or(Some(&0)).unwrap()+val;
            next_timeline.insert(key, new_val);
        }
    }
    next_timeline
}

fn propagate_quantum_tachyon(matrix: &Vec<Vec<u8>>, line: usize, col: usize) -> usize {
    let mut timelines: HashMap<(usize, usize), usize> = HashMap::from([((line, col), 1)]);

    for _ in line..matrix.len() {
        timelines = propagate_quantum_tachyon_timeline(matrix, &timelines);
    }
    
    let mut result = 0;
    for (_, count) in timelines {
        result += count;
    }

    result
}

#[test]
fn test_propagate_quantum_tachyon() {
        let matrix =[
".......S.......".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
".......^.......".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
"......^.^......".as_bytes().to_vec(),
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
    assert_eq!(timeline, 4);


            let matrix =[
".......S.......".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
".......^.......".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
"......^.^......".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
".....^.^.^.....".as_bytes().to_vec(),
"...............".as_bytes().to_vec(),
].to_vec();

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
    assert_eq!(timeline, 8);

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
    assert_eq!(40, timeline);
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
