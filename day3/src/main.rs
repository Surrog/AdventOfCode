use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn compute_joltage(input: &str, size: usize) -> u64 {
    let input_byte = input.as_bytes();
    if input.len() < size {
        panic!("trying to find a bigger number than the input value")
    }

    let mut result_str = Vec::new();
    result_str.resize(size, b'0');

    let mut search_begin: usize = 0;
    for result_i in 0..result_str.len() {
        let search_limit = input_byte.len() - result_str.len() + result_i + 1;
        for input_i in search_begin..search_limit {
            if input_byte[input_i] > result_str[result_i] {
                result_str[result_i] = input_byte[input_i];
                search_begin = input_i + 1;
            }
        }
    }
    return String::from_utf8(result_str)
        .expect("not a uf8 string")
        .parse()
        .expect("cannot convert it to number");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_joltage() {
        assert_eq!(compute_joltage("987654321111111", 2), 98);
        assert_eq!(compute_joltage("811111111111119", 2), 89);
        assert_eq!(compute_joltage("234234234234278", 2), 78);
        assert_eq!(compute_joltage("818181911112111", 2), 92);

        assert_eq!(compute_joltage("987654321111111", 12), 987654321111);
        assert_eq!(compute_joltage("811111111111119", 12), 811111111119);
        assert_eq!(compute_joltage("234234234234278", 12), 434234234278);
        assert_eq!(compute_joltage("818181911112111", 12), 888911112111);
    }
}

fn main() -> io::Result<()> {
    let file =
        File::open(env::current_dir()?.join("src/input.txt")).expect("failed to read input.txt");
    let reader = BufReader::new(file);

    let mut sum_joltage = 0;
    for line in reader.lines() {
        sum_joltage += compute_joltage(line?.trim(), 12)
    }
    println!("sum of joltage: {sum_joltage}");

    Ok(())
}
