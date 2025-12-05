use std::{
    cmp::{max, min},
    env,
    fmt::Debug,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct RangeId {
    min: u64,
    max: u64,
}

#[derive(Debug, Clone)]
struct LineParseError {
    badline: String,
}

impl RangeId {
    fn from_str(line: &str) -> Result<RangeId, LineParseError> {
        let option = line.split_once('-');
        match option {
            None => {
                return Err(LineParseError {
                    badline: format!("failed to parse {}", line),
                });
            }
            Some((lval, rval)) => {
                let lnum = lval.parse().map_err(|_| LineParseError {
                    badline: format!("failed to parse {}", lval),
                })?;
                let rnum = rval.parse().map_err(|_| LineParseError {
                    badline: format!("failed to parse {}", lval),
                })?;
                return Ok(RangeId {
                    min: min(lnum, rnum),
                    max: max(lnum, rnum),
                });
            }
        };
    }
}

fn in_range(range: &RangeId, id: u64) -> bool {
    return id >= range.min && id <= range.max;
}

fn merge_range(lval: &RangeId, rval: &RangeId) -> Option<RangeId> {
    if in_range(lval, rval.min) || in_range(rval, lval.min) {
        return Some(RangeId {
            min: min(lval.min, rval.min),
            max: max(lval.max, rval.max),
        });
    }

    None
}

fn consolidate_ranges_once(ids: &Vec<RangeId>, idx: usize) -> Vec<RangeId> {
    let mut result: Vec<RangeId> = ids[0..idx].to_vec();
    result.reserve(ids.len());

    let mut current = ids[idx].clone();
    for i in idx + 1..ids.len() {
        let merg = merge_range(&current, &ids[i]);
        match merg {
            Some(m) => current = m,
            None => result.push(ids[i].clone()),
        };
    }
    result.insert(idx, current);
    return result;
}

fn consolidate_ranges(mut ids: Vec<RangeId>) -> Vec<RangeId> {
    let mut idx: usize = 0;
    loop {
        if idx >= ids.len() {
            break;
        }
        let last_len = ids.len();
        ids = consolidate_ranges_once(&ids, idx);
        if ids.len() == last_len {
            idx += 1;
        }
    }
    return ids;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_range() {
        assert_eq!(in_range(&RangeId { min: 3, max: 5 }, 0), false);
        assert_eq!(in_range(&RangeId { min: 3, max: 5 }, 1), false);
        assert_eq!(in_range(&RangeId { min: 3, max: 5 }, 2), false);
        assert_eq!(in_range(&RangeId { min: 3, max: 5 }, 3), true);
        assert_eq!(in_range(&RangeId { min: 3, max: 5 }, 4), true);
        assert_eq!(in_range(&RangeId { min: 3, max: 5 }, 5), true);
        assert_eq!(in_range(&RangeId { min: 3, max: 5 }, 6), false);
        assert_eq!(in_range(&RangeId { min: 3, max: 5 }, 7), false);
    }

    #[test]
    fn test_range_from_str() {
        assert_eq!(
            RangeId::from_str("1-2").expect("1-2 should be parsed"),
            RangeId { min: 1, max: 2 }
        );

        assert_eq!(
            RangeId::from_str("2-1").expect("2-1 should be parsed"),
            RangeId { min: 1, max: 2 }
        );

        assert_eq!(
            RangeId::from_str("1233-35566").expect("1233-35566 should be parsed"),
            RangeId {
                min: 1233,
                max: 35566
            }
        );

        assert!(RangeId::from_str("1233-").is_err());
        assert!(RangeId::from_str("-1233").is_err());
        assert!(RangeId::from_str("-").is_err());
        assert!(RangeId::from_str("azer-234").is_err());
        assert!(RangeId::from_str("azer---def").is_err());
        assert!(RangeId::from_str("12-23-34").is_err());
    }

    #[test]
    fn test_merge_range() {
        let result = merge_range(&RangeId { min: 3, max: 5 }, &RangeId { min: 10, max: 14 });
        assert!(result.is_none());

        let result = merge_range(&RangeId { min: 3, max: 5 }, &RangeId { min: 5, max: 14 });
        assert!(result.is_some());
        assert_eq!(result.unwrap(), RangeId { min: 3, max: 14 });

        let result = merge_range(&RangeId { min: 13, max: 23 }, &RangeId { min: 5, max: 14 });
        assert!(result.is_some());
        assert_eq!(result.unwrap(), RangeId { min: 5, max: 23 });

        let result = merge_range(&RangeId { min: 13, max: 23 }, &RangeId { min: 15, max: 20 });
        assert!(result.is_some());
        assert_eq!(result.unwrap(), RangeId { min: 13, max: 23 });
    }

    #[test]
    fn test_consolidate_once() {
        let values = vec![RangeId { min: 3, max: 5 }];

        let result = consolidate_ranges_once(&values, 0);
        assert_eq!(values, result);

        let values2 = vec![RangeId { min: 3, max: 5 }, RangeId { min: 3, max: 9 }];
        let expect2 = vec![RangeId { min: 3, max: 9 }];
        let result = consolidate_ranges_once(&values2, 0);

        assert_eq!(expect2, result);

        let values2 = vec![RangeId { min: 3, max: 5 }, RangeId { min: 6, max: 9 }];
        let expect2 = vec![RangeId { min: 3, max: 5 }, RangeId { min: 6, max: 9 }];
        let result = consolidate_ranges_once(&values2, 0);

        assert_eq!(expect2, result);

        let values2 = vec![
            RangeId { min: 3, max: 5 },
            RangeId { min: 5, max: 13 },
            RangeId { min: 6, max: 9 },
        ];
        let expect2 = vec![RangeId { min: 3, max: 5 }, RangeId { min: 5, max: 13 }];
        let result = consolidate_ranges_once(&values2, 1);
        assert_eq!(expect2, result);

        let expect2 = vec![RangeId { min: 3, max: 13 }];
        let result = consolidate_ranges_once(&result, 0);
        assert_eq!(expect2, result);

        let values2 = vec![
            RangeId { min: 3, max: 5 },
            RangeId { min: 5, max: 13 },
            RangeId { min: 6, max: 9 },
        ];
        let expect2 = vec![RangeId { min: 3, max: 13 }];
        let result = consolidate_ranges_once(&values2, 0);
        assert_eq!(expect2, result);
    }

    #[test]
    fn test_consolidate() {
        let values2 = vec![
            RangeId { min: 3, max: 5 },
            RangeId { min: 6, max: 9 },
            RangeId { min: 5, max: 13 },
        ];
        let expect2 = vec![RangeId { min: 3, max: 13 }];
        let result = consolidate_ranges(values2);
        assert_eq!(expect2, result);
    }
}

fn main() -> io::Result<()> {
    let file = File::open(env::current_dir()?.join("src/input.txt"))?;
    let reader = BufReader::new(file);

    let mut sum_indredient_fresh = 0;
    let mut valid_range_ids = Vec::new();
    let mut parsing_range = true;
    for line_result in reader.lines() {
        let line = line_result?;
        if line.len() == 0 {
            parsing_range = false;
            valid_range_ids = consolidate_ranges(valid_range_ids);
            continue;
        }
        if parsing_range {
            valid_range_ids.push(
                RangeId::from_str(line.as_str())
                    .expect(format!("failed to parse line {}", line).as_str()),
            );
        } else {
            let id_to_check: u64 = line
                .parse()
                .expect(format!("failed to parse line {}", line).as_str());
            let mut is_valid = false;

            for range in &valid_range_ids {
                is_valid = is_valid || in_range(&range, id_to_check)
            }
            if is_valid {
                sum_indredient_fresh += 1;
            }
        }
    }
    let mut sum_id_fresh = 0;

    for range in &valid_range_ids {
        sum_id_fresh += range.max - range.min + 1;
    }
    println!("total number of fresh ingretient: {}", sum_indredient_fresh);
    println!("total number of fresh ids: {}", sum_id_fresh);
    Ok(())
}
