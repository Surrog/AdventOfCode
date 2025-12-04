use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn line_contain_roll(line: &[u8], index: usize) -> u64 {
    let mut count = 0;
    if index < line.len() && index > 0 && line[index-1] == b'@' {
        count += 1;
    }
    if index < line.len() && line[index] == b'@' {
        count +=1;
    }
    if line.len() > 0 && index < line.len() - 1 && line[index+1] == b'@' {
        count += 1;
    }
    return count
}

fn process_line(local_grid: &Vec<Vec<u8>>, local_index: usize) -> (u64, Vec<u8>) {
    let mut cleaned_line = local_grid[local_index].clone();
    let mut line_roll_count = 0; 
    
    let length = cleaned_line.len();
    for i in 0..length {
        if cleaned_line[i] == b'@' {
            let mut count = 0;
            // check previous line for roll
            if local_index > 0 {
                count += line_contain_roll(local_grid[local_index-1].as_slice(), i);
            }
            // check after for roll
            count += line_contain_roll(local_grid[local_index].as_slice(), i);
            // check next line for roll
            if local_index + 1 < local_grid.len() {
                count += line_contain_roll(local_grid[local_index+1].as_slice(), i);
            } 
            if count < 5 {
                line_roll_count += 1;
                cleaned_line[i]=b'x';
            }                    
        }
    }
    (line_roll_count, cleaned_line)
}

fn process_floor(roll_map: &Vec<Vec<u8>>) -> (u64, Vec<Vec<u8>>) {
    let mut total_roll_found: u64 = 0; 
    let mut processed_map: Vec<Vec<u8>>= Vec::new();
    processed_map.reserve(roll_map.len());

    for local_index  in 0..roll_map.len() {
        let (line_roll_count, new_line) = process_line(roll_map, local_index);

        total_roll_found += line_roll_count;
        processed_map.push(new_line);
    }
    
    (total_roll_found, processed_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_contain_roll() {
        let line = "..@@.@@@@.";
        let expected: [u64; 10] = [0, 1, 2, 2, 2, 2, 3, 3, 2, 1];
        for i in 0..line.len() {
            let result = line_contain_roll(line.as_bytes(), i);
            assert_eq!(result, expected[i]);
        }
        line_contain_roll("".as_bytes(), 1);
    }

    #[test]
    fn test_process_line() {
        let local_grid= ["..@@.@@@@.".as_bytes().to_vec(), "@@@.@.@.@@".as_bytes().to_vec(), "@@@@@.@.@@".as_bytes().to_vec(), "@.@@@@..@.".as_bytes().to_vec()].to_vec();

        let (result,line) = process_line(&local_grid, 0);
        assert_eq!(result, 5);
        assert_eq!(line, "..xx.xx@x.".as_bytes().to_vec());
        let (result,line) = process_line(&local_grid, 1);
        assert_eq!(result, 1);
        assert_eq!(line, "x@@.@.@.@@".as_bytes().to_vec());
        let (result,line) = process_line(&local_grid, 2);
        assert_eq!(result, 1);
        assert_eq!(line, "@@@@@.x.@@".as_bytes().to_vec());
    }

    #[test]
    fn test_compute_floor() {
        let reader = io::Cursor::new(r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#);
        let mut roll_map: Vec<Vec<u8>> = Vec::new();

        for line in reader.lines() {
            roll_map.push(Vec::from(line.expect("properly read line").as_bytes()));
        }
        
        let (total_roll_found, _) = process_floor(&roll_map);
        assert_eq!(total_roll_found, 13)
    }
}


fn main() -> io::Result<()> {
    let file =
        File::open(env::current_dir()?.join("src/input.txt"))?;
    let reader = BufReader::new(file);

    let mut roll_map: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        roll_map.push(Vec::from(line?.as_bytes()));
    }

    let mut total_roll_processed = 0;
    loop {
        let (roll_processed, new_roll_map) = process_floor(&roll_map);
        roll_map = new_roll_map;
        total_roll_processed += roll_processed;
        if roll_processed == 0 {
            break;
        }
    }

    println!("stuck rolled: {total_roll_processed}");

    Ok(())
}
