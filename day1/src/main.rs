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

fn roll_value(current: i32, val: i32) -> (i32, i32) {
    let mut result = current + val;
    let mut clicks = (result / 100).abs();
    if current > 0 && result <= 0 {
        clicks += 1;
    }
    result = result.rem_euclid(100);
    (result, clicks)
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
        (cursor, _) = roll_value(cursor, -68);
        assert_eq!(cursor, 82);

        (cursor, _) = roll_value(cursor, -30);
        assert_eq!(cursor, 52);

        (cursor, _) = roll_value(cursor, 48);
        assert_eq!(cursor, 0);

        (cursor, _) = roll_value(cursor, -5);
        assert_eq!(cursor, 95);

        (cursor, _) = roll_value(cursor, 60);
        assert_eq!(cursor, 55);

        (cursor, _) = roll_value(cursor, -55);
        assert_eq!(cursor, 0);

        (cursor, _) = roll_value(cursor, -1);
        assert_eq!(cursor, 99);

        (cursor, _) = roll_value(cursor, -99);
        assert_eq!(cursor, 0);

        (cursor, _) = roll_value(cursor, 14);
        assert_eq!(cursor, 14);

        (cursor, _) = roll_value(cursor, -82);
        assert_eq!(cursor, 32);

        (cursor, _) = roll_value(0, 100);
        assert_eq!(cursor, 0);
    }

    #[test]
    fn test_clicks() {
        let mut cursor = 50;
        let mut click;
        (cursor, click) = roll_value(cursor, 50);
        assert_eq!(cursor, 0);
        assert_eq!(click, 1);
        (cursor, click) = roll_value(cursor, 50);
        assert_eq!(cursor, 50);
        assert_eq!(click, 0);
        (cursor, click) = roll_value(cursor, 50);
        assert_eq!(cursor, 0);
        assert_eq!(click, 1);
        (cursor, click) = roll_value(cursor, 50);
        assert_eq!(cursor, 50);
        assert_eq!(click, 0);
        (cursor, click) = roll_value(cursor, 100);
        assert_eq!(cursor, 50);
        assert_eq!(click, 1);
        (cursor, click) = roll_value(cursor, -100);
        assert_eq!(cursor, 50);
        assert_eq!(click, 1);
        (cursor, click) = roll_value(cursor, -50);
        assert_eq!(cursor, 0);
        assert_eq!(click, 1);
        (cursor, click) = roll_value(50, -150);
        assert_eq!(cursor, 0);
        assert_eq!(click, 2);
        (cursor, click) = roll_value(50, -250);
        assert_eq!(cursor, 0);
        assert_eq!(click, 3);
    }
}

fn main() -> io::Result<()> {
    let file =
        File::open(env::current_dir()?.join("src/input.txt")).expect("failed to read input.txt");
    let reader = BufReader::new(file);

    let mut zero_counter = 0;
    let mut cursor = 50;
    let mut clicks;
    for line in reader.lines() {
        (cursor, clicks) = roll_value(cursor, parse_line(line?));
        zero_counter += clicks;
    }
    println!("found counter: {zero_counter}");

    Ok(())
}
