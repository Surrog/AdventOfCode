use core::num;
use std::{
    cmp::max,
    collections::HashMap,
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Add,
};

fn add(lval: i64, rval: i64) -> i64 {
    return lval + rval;
}

fn sub(lval: i64, rval: i64) -> i64 {
    0
}

fn mul(lval: i64, rval: i64) -> i64 {
    return lval * rval;
}

fn div(lval: i64, rval: i64) -> i64 {
    0
}

fn main() -> io::Result<()> {
    let file = File::open(env::current_dir()?.join("src/input.txt"))?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<i64>> = Vec::new();
    let mut op: Vec<u8> = Vec::new();
    let mut expected_op: HashMap<u8, fn(i64, i64) -> i64> = HashMap::new();
    expected_op.insert(b'+', add);
    expected_op.insert(b'-', sub);
    expected_op.insert(b'*', mul);
    expected_op.insert(b'/', div);

    for line in reader.lines() {
        let mut num_line: Vec<i64> = Vec::new();

        for tok in line?.split_whitespace() {
            let bytes = tok.as_bytes();
            if bytes.len() == 1 && expected_op.contains_key(&bytes[0]) {
                op.push(bytes[0]);
            } else {
                num_line.push(tok.parse().map_err(|err| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("could not parse token: '{tok}' into number: {err}"),
                    )
                })?);
            }
        }

        if num_line.len() > 0 {
            matrix.push(num_line);
        }
    }

    for line in 1..matrix.len() {
        for col in 0..matrix[line].len() {
            matrix[0][col] = expected_op[&op[col]](matrix[0][col], matrix[line][col]);
        }
    }
    let result: i64 = matrix[0].iter().sum();

    println!("grand total of operation: {result}");

    Ok(())
}
