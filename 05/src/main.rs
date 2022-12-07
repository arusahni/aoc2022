use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2022-05")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

struct Move {
    count: u32,
    source: u32,
    dest: u32,
}

fn parse_move(moveline: &String) -> Move {
    lazy_static! {
        static ref MOVE_RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }
    let numbers = MOVE_RE.captures(moveline).unwrap();
    Move {
        count: numbers[1].parse::<u32>().unwrap(),
        source: numbers[2].parse::<u32>().unwrap(),
        dest: numbers[3].parse::<u32>().unwrap(),
    }
}

fn format_stacks(stacks: &Vec<Vec<char>>) -> String {
    let mut output = vec![Range {
        start: 1,
        end: 1 + stacks.len(),
    }
    .map(|i| format!(" {i} "))
    .collect::<Vec<String>>()
    .join(" ")];
    let max_stack_length = stacks
        .iter()
        .map(|s| s.len())
        .max()
        .expect("Could not find max stack length");
    for x in 0..max_stack_length {
        output.push(
            stacks
                .iter()
                .map(|s| {
                    s.get(x)
                        .map_or_else(|| String::from("   "), |c| format!("[{c}]"))
                })
                .collect::<Vec<String>>()
                .join(" "),
        );
    }
    output.reverse();
    output.join("\n")
}

fn main() {
    let opt = Opt::from_args();
    let file = File::open(&opt.input).unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|s| s.unwrap()).collect();
    let mut stack_header: Vec<_> = lines
        .iter()
        .filter(|f| !f.starts_with("m") && f.len() != 0)
        .collect();
    stack_header.reverse();
    let total_stacks = stack_header
        .first()
        .unwrap()
        .chars()
        .filter(|c| c.is_numeric())
        .count();
    let stack_lines = stack_header[1..].to_vec();
    let mut stacks_single: Vec<_> = std::iter::repeat(Vec::<char>::new())
        .take(total_stacks)
        .collect();
    stack_lines
        .iter()
        .filter(|f| !f.starts_with("m") && f.len() != 0)
        .for_each(|line| {
            let mut stack_index = 0;
            for (ix, character) in line.char_indices() {
                match character {
                    '[' | ']' | ' ' => (),
                    c => {
                        stacks_single[stack_index].push(c.clone());
                    }
                }
                if ix % 4 == 3 {
                    stack_index += 1;
                }
            }
        });
    let mut stacks_multiple = stacks_single.clone();
    let moves: Vec<Move> = lines
        .iter()
        .filter(|l| l.starts_with('m'))
        .map(parse_move)
        .collect();
    for step in moves {
        let single_source_stack = stacks_single
            .get_mut(step.source as usize - 1)
            .expect("Couldn't find source stack");
        let multiple_source_stack = stacks_multiple
            .get_mut(step.source as usize - 1)
            .expect("Couldn't find source stack");
        let mut to_move_single: Vec<char> = single_source_stack
            .drain((single_source_stack.len() - step.count as usize)..)
            .collect();
        to_move_single.reverse();
        let mut to_move_multiple: Vec<char> = multiple_source_stack
            .drain((multiple_source_stack.len() - step.count as usize)..)
            .collect();
        stacks_multiple
            .get_mut(step.dest as usize - 1)
            .expect("Couldn't find dest stack")
            .append(&mut to_move_multiple);
        stacks_single
            .get_mut(step.dest as usize - 1)
            .expect("Couldn't find dest stack")
            .append(&mut to_move_single);
    }
    let single_formatted = format_stacks(&stacks_single);
    let single_tops = String::from_iter(stacks_single.iter().map(|s| s.last().unwrap()));
    let multiple_formatted = format_stacks(&stacks_multiple);
    let multiple_tops = String::from_iter(stacks_multiple.iter().map(|s| s.last().unwrap()));
    println!("SINGLE:\n{single_formatted}");
    println!("Tops: {single_tops}");
    println!("MULTIPLE:\n{multiple_formatted}");
    println!("Tops: {multiple_tops}");
}
