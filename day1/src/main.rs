use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn parse_line(line: String) -> i32 {
    let mut value: i32 = line[1..].parse().expect("failed to convert string to int");
    if line.starts_with('L') {
        value *= -1;
    }
    value
}

fn roll_value(current: i32, val: i32) -> i32 {
    let mut result = current + val;
    if result < 0 {
        result = 100 + result;
    }
    result %= 100;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("L68".to_string()), -68);
        assert_eq!(parse_line("L30".to_string()), -30);
        assert_eq!(parse_line("R48".to_string()), 48);
        assert_eq!(parse_line("L5".to_string()), -5);
        assert_eq!(parse_line("R60".to_string()), 60);
    }

    #[test]
    fn test_roll_value() {
        let mut cursor = 50;
        cursor = roll_value(cursor, -68);
        assert_eq!(cursor, 82);
        cursor = roll_value(cursor, -30);
        assert_eq!(cursor, 52);
        cursor = roll_value(cursor, 48);
        assert_eq!(cursor, 0);
        cursor = roll_value(cursor, -5);
        assert_eq!(cursor, 95);
        cursor = roll_value(cursor, 60);
        assert_eq!(cursor, 55);
        cursor = roll_value(cursor, -55);
        assert_eq!(cursor, 0);
        cursor = roll_value(cursor, -1);
        assert_eq!(cursor, 99);
        cursor = roll_value(cursor, -99);
        assert_eq!(cursor, 0);
        cursor = roll_value(cursor, 14);
        assert_eq!(cursor, 14);
        cursor = roll_value(cursor, -82);
        assert_eq!(cursor, 32);
    }
}

fn main() -> io::Result<()> {
    let file =
        File::open(env::current_dir()?.join("src/input.txt")).expect("failed to read input.txt");
    let reader = BufReader::new(file);

    let mut line_count = 0;
    let mut zero_counter = 0;
    let mut cursor = 50;
    for line in reader.lines() {
        cursor = roll_value(cursor, parse_line(line?));

        if cursor == 0 {
            zero_counter += 1;
        }
        line_count += 1;
    }
    println!("found counter: {zero_counter}");

    Ok(())
}
