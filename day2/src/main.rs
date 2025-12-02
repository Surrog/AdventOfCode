use std::{env};
use std::fs::File;
use std::io::{self, BufReader, Read};

fn detect_bad_id(id: &str) -> bool {
    let len = id.len();
    let mut split_str = Vec::new();
    for n in 2..len+1 {
        if len % n == 0 {
            let div = len/n;
            split_str.clear();
            for i in 0..n {
                let start = i*div;
                let end = start + div; 
                split_str.push(&id[start..end]);
            }

            if split_str.iter().all(|&item| item == split_str[0]) {
                return true
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_bad_id() {
        assert!(detect_bad_id("11"));
        assert!(detect_bad_id("22"));
        assert!(detect_bad_id("99"));
        assert!(detect_bad_id("1010"));
        assert!(detect_bad_id("1188511885"));
        assert!(detect_bad_id("222222"));
        assert!(detect_bad_id("446446"));
        assert!(detect_bad_id("38593859"));
        assert!(detect_bad_id("111"));
        assert!(detect_bad_id("999"));
        assert!(detect_bad_id("565656"));
        assert!(detect_bad_id("824824824"));
        assert!(detect_bad_id("2121212121"));
        assert!(!detect_bad_id("12"));
        assert!(!detect_bad_id("112"));
        assert!(!detect_bad_id("1234567890"));
        assert!(!detect_bad_id("222220"));
        assert!(!detect_bad_id("222221"));
        assert!(!detect_bad_id("222223"));
        assert!(!detect_bad_id("222224"));
    }
}

fn main() -> io::Result<()> {
    let file =
        File::open(env::current_dir()?.join("src/input.txt")).expect("failed to read input.txt");
    let mut reader = BufReader::new(file);

    let mut sum_id = 0;
    let mut ids = String::new(); 
    reader.read_to_string(&mut ids)?;
    for range in ids.split(',') {
        let (begin, end) = range.trim().split_once('-').expect("missing - in ids");
        let num_begin: u64 = begin.parse().expect("failed to cast id to int");
        let num_end: u64 = end.parse().expect("failed to cast id to int");

        for n in num_begin..num_end+1 {
            if detect_bad_id(n.to_string().as_str()) {
                sum_id += n
            }
        }
    }
    println!("sum of bad ids: {sum_id}");

    Ok(())
}
