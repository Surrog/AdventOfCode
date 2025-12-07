use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn process_sub_matrix(mat: &Vec<Vec<u8>>, begin: usize, end: usize) -> Result<i64, io::Error> {
    let num_max_size = mat.len();
    let op = mat.last().unwrap()[begin];
    let mut acc: i64 = 0;
    let mut num_bytes = Vec::new();
    num_bytes.reserve(num_max_size);

    for col in begin..end {
        num_bytes.clear();
        for line in 0..mat.len() - 1 {
            if mat[line][col] != b' ' {
                num_bytes.push(mat[line][col]);
            }
        }
        let num = str::from_utf8(num_bytes.as_slice())
            .map_err(|err| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("failed to recast into str: {err}"),
                )
            })?
            .parse()
            .map_err(|err| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("failed to parse into number: {err}"),
                )
            })?;
        if acc == 0 {
            acc = num
        } else {
            match op {
                b'+' => acc += num,
                b'*' => acc *= num,
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("unexepected op '{op}'"),
                    ));
                }
            }
        }
    }

    Ok(acc)
}

#[test]
fn test_process_sub_matric() {
    let matrix: Vec<Vec<u8>> = vec![vec![b'1'], vec![b'2'], vec![b'2'], vec![b'+']];

    let result = process_sub_matrix(&matrix, 0, 1);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 122);

    let matrix: Vec<Vec<u8>> = vec![
        vec![b'1', b'2'],
        vec![b'2', b' '],
        vec![b'2', b' '],
        vec![b'+', b' '],
    ];

    let result = process_sub_matrix(&matrix, 0, 2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 124);

    let matrix: Vec<Vec<u8>> = vec![
        vec![b'1', b'2'],
        vec![b'2', b'2'],
        vec![b'2', b' '],
        vec![b'+', b' '],
    ];

    let result = process_sub_matrix(&matrix, 0, 2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 144);

    let matrix: Vec<Vec<u8>> = vec![
        vec![b'1', b'2'],
        vec![b'2', b'2'],
        vec![b'2', b' '],
        vec![b'*', b' '],
    ];

    let result = process_sub_matrix(&matrix, 0, 2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 122 * 22);

    let matrix: Vec<Vec<u8>> = vec![
        vec![b'1', b'2'],
        vec![b'2', b'a'],
        vec![b'2', b' '],
        vec![b'*', b' '],
    ];

    let result = process_sub_matrix(&matrix, 0, 2);
    assert!(result.is_err());

    let matrix: Vec<Vec<u8>> = vec![
        vec![b'1', b'2'],
        vec![b'2', b'2'],
        vec![b'2', b' '],
        vec![b'/', b' '],
    ];

    let result = process_sub_matrix(&matrix, 0, 2);
    assert!(result.is_err());
}

fn main() -> io::Result<()> {
    let file = File::open(env::current_dir()?.join("src/input.txt"))?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        matrix.push(line?.as_bytes().to_vec());
    }

    let mut result: i64 = 0;
    if matrix.len() == 0 {
        return Ok(());
    }

    let mut begin = 0;
    let op = matrix.last().unwrap();
    for tok_index in 1..op.len() {
        if op[tok_index] != b' ' {
            result += process_sub_matrix(&matrix, begin, tok_index - 1)?;
            begin = tok_index;
        }
    }
    //process last matrix
    result += process_sub_matrix(&matrix, begin, op.len())?;

    println!("grand total of operation: {result}");

    Ok(())
}
