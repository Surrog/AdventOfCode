use std::{
    env,
    fs::File,
    i64,
    io::{self, BufRead, BufReader},
};

struct Machine {
    lights: u32,
    buttons: Vec<Vec<u32>>,
    joltage: Vec<u32>,
}

fn push_button_light(mut current: u32, button: &Vec<u32>) -> u32 {
    for val in button {
        current = current ^ (1 << val);
    }
    current
}

#[test]
fn test_push_button_light() {
    assert_eq!(push_button_light(0, &Vec::new()), 0);

    assert_eq!(push_button_light(0, &vec![0]), 1);
    assert_eq!(push_button_light(0, &vec![1]), 2);
    assert_eq!(push_button_light(0, &vec![2]), 4);
    assert_eq!(push_button_light(0, &vec![1, 2]), 6);
    assert_eq!(push_button_light(0, &vec![0, 1]), 3);

    assert_eq!(push_button_light(4, &vec![2]), 0);
    assert_eq!(push_button_light(2, &vec![2]), 6);
}

fn push_button_joltage(current: &Vec<u32>, button: &Vec<u32>, output: &mut Vec<u32>) {
    for val in button {
        output[*val as usize] = current[*val as usize] + 1;
    }
}

#[test]
fn test_push_button_joltage() {
    let current = vec![0; 10];
    let mut res = vec![0; 10];

    push_button_joltage(&current, &vec![1, 2, 5], &mut res);
    assert_eq!(res[1], 1);
    assert_eq!(res[2], 1);
    assert_eq!(res[5], 1);

    let current = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    push_button_joltage(&current, &vec![1, 2, 5], &mut res);
    assert_eq!(res[1], 2);
    assert_eq!(res[2], 3);
    assert_eq!(res[5], 6);
}

fn search_button(mach: &Machine, current: u32, deep: usize) -> bool {
    for i in 0..mach.buttons.len() {
        let updated = push_button_light(current, &mach.buttons[i]);
        if updated == mach.lights {
            return true;
        }
        if deep > 0 {
            if search_button(mach, updated, deep - 1) {
                return true;
            }
        }
    }
    return false;
}

#[test]
fn test_search_button() {
    let mach = &Machine {
        lights: 5,
        buttons: vec![vec![0], vec![1], vec![2]],
        joltage: vec![],
    };

    let found = search_button(mach, 0, 0);
    assert_eq!(found, false);

    let found = search_button(mach, 0, 1);
    assert_eq!(found, true);
}

fn search_button_list(mach: &Machine) -> usize {
    let mut deep: usize = 0;

    loop {
        if search_button(mach, 0, deep) {
            return deep + 1;
        }

        deep += 1;
    }
}

#[test]
fn test_search_button_list() {
    let mach = &Machine {
        lights: 5,
        buttons: vec![vec![0], vec![1], vec![2]],
        joltage: vec![],
    };

    let found = search_button_list(mach);
    assert_eq!(found, 2);

    let mach = &Machine {
        lights: parse_light("[.##.]"),
        buttons: vec![
            vec![3],
            vec![1, 3],
            vec![2],
            vec![2, 3],
            vec![0, 2],
            vec![0, 1],
        ],
        joltage: vec![],
    };

    let found = search_button_list(mach);
    assert_eq!(found, 2);

    let mach = &Machine {
        lights: parse_light("[...#.]"),
        buttons: vec![
            vec![0, 2, 3, 4],
            vec![2, 3],
            vec![0, 4],
            vec![0, 1, 2],
            vec![1, 2, 3, 4],
        ],
        joltage: vec![],
    };

    let found = search_button_list(mach);
    assert_eq!(found, 3);

    let mach = &Machine {
        lights: parse_light("[.###.#]"),
        buttons: vec![
            vec![0, 1, 2, 3, 4],
            vec![0, 3, 4],
            vec![0, 1, 2, 4, 5],
            vec![1, 2],
        ],
        joltage: vec![],
    };

    let found = search_button_list(mach);
    assert_eq!(found, 2);
}

fn search_joltage(mach: &Machine, current: &Vec<u32>, deep: usize) -> bool {
    let mut working_value = current.clone();
    for i in 0..mach.buttons.len() {
        working_value.copy_from_slice(&current);
        push_button_joltage(&current, &mach.buttons[i], &mut working_value);
        if working_value == mach.joltage {
            return true;
        }
        if deep > 0 {
            if search_joltage(mach, &working_value, deep - 1) {
                return true;
            }
        }
    }
    return false;
}

fn search_joltage_list(mach: &Machine) -> usize {
    let mut deep: usize = 0;
    let mut current = mach.joltage.clone();
    current.fill(0);

    loop {
        if search_joltage(mach, &current, deep) {
            return deep + 1;
        }

        deep += 1;
    }
}

fn main() -> io::Result<()> {
    let file = File::open(env::current_dir()?.join("src/input.txt"))?;
    let reader = BufReader::new(file);

    let mut machines = Vec::new();
    for l in reader.lines() {
        let mut mach: Machine = Machine {
            lights: 0,
            buttons: Vec::new(),
            joltage: Vec::new(),
        };
        for elem in l?.split(' ') {
            if elem.starts_with('[') {
                mach.lights = parse_light(elem);
            }
            if elem.starts_with('(') {
                mach.buttons.push(parse_button(elem)?);
            }
            if elem.starts_with('{') {
                mach.joltage = parse_button(elem)?;
            }
        }
        machines.push(mach);
    }

    let mut button_sum = 0;
    for i in 0..machines.len() {
        println!("searching machine {}/{}", i, machines.len());
        button_sum += search_button_list(&machines[i]);
    }
    println!("sum of total light button: {}", button_sum);

    let mut button_sum = 0;
    for i in 0..machines.len() {
        println!("searching machine {}/{}", i, machines.len());
        button_sum += search_joltage_list(&machines[i]);
    }
    println!("sum of total joltage button: {}", button_sum);

    Ok(())
}

fn parse_light(elem: &str) -> u32 {
    let mut result = 0;
    let bytes = elem.as_bytes();
    for c in 1..bytes.len() {
        if bytes[c] == b'#' {
            result = result | (1 << (c - 1));
        }
    }
    result
}

#[test]
fn test_parse_light() {
    let res = parse_light("[.##.]");
    assert_eq!(res, 2 + 4);

    let res = parse_light("[...#.]");
    assert_eq!(res, 8);
}

fn parse_button(elem: &str) -> io::Result<Vec<u32>> {
    let mut button_num = Vec::new();
    for num in elem[1..(elem.len() - 1)].split(',') {
        button_num.push(num.parse().map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("failed to parse {} into a number: {}", num, err),
            )
        })?);
    }
    Ok(button_num)
}

#[test]
fn test_parse_button() {
    let res = parse_button("(1,2,3)");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vec![1, 2, 3]);

    let res = parse_button("{1,2,3}");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vec![1, 2, 3]);
}
