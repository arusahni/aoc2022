use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use serde_json::{json, Value};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2022-13")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn compare_packets(left: &Vec<Value>, right: &Vec<Value>) -> Ordering {
    for pair in left.iter().zip_longest(right.iter()) {
        match pair {
            Left(_) => return Ordering::Greater,
            Right(_) => return Ordering::Less,
            Both(left_item, right_item) => {
                let mut left_val = left_item.to_owned();
                let mut right_val = right_item.to_owned();
                if left_val.is_number() && right_val.is_array() {
                    left_val = json!([left_val]);
                } else if left_val.is_array() && right_val.is_number() {
                    right_val = json!([right_val]);
                }
                let result = if left_val.is_array() {
                    compare_packets(left_val.as_array().unwrap(), right_val.as_array().unwrap())
                } else if left_val.is_number() {
                    left_val.as_u64().unwrap().cmp(&right_val.as_u64().unwrap())
                } else {
                    panic!("Unknown type");
                };
                match result {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => (),
                };
            }
        }
    }
    Ordering::Equal
}

fn main() {
    let opt = Opt::from_args();
    let file = File::open(&opt.input).unwrap();
    let lines: Vec<_> = BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| serde_json::from_str::<Vec<Value>>(&s).unwrap())
        .collect::<Vec<_>>();

    // Part 1
    let index_sum: usize = lines
        .chunks(2)
        .map(|packets| !matches!(compare_packets(&packets[0], &packets[1]), Ordering::Greater))
        .enumerate()
        .filter(|(_, eq)| *eq)
        .map(|(ix, _)| ix + 1)
        .sum();

    // Part 2
    let divider1 = vec![json!([2])];
    let divider2 = vec![json!([6])];
    let mut sorted = vec![divider1.clone(), divider2.clone()];
    sorted.extend(lines);
    sorted.sort_by(compare_packets);
    let decoder_key: usize = sorted
        .iter()
        .enumerate()
        .filter(|(_ix, p)| {
            matches!(compare_packets(p, &divider1), Ordering::Equal)
                || matches!(compare_packets(p, &divider2), Ordering::Equal)
        })
        .map(|(ix, _)| ix + 1)
        .product();

    println!("Sum: {index_sum}");
    println!("Decoder key: {decoder_key}");
}
