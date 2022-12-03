use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2022-03")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn prioritize_item(item: char) -> u32 {
    item as u32
        - match item {
            'A'..='Z' => 38,
            'a'..='z' => 96,
            _ => panic!("Unexpected item {item}"),
        }
}

fn main() {
    let opt = Opt::from_args();
    let file = File::open(&opt.input).unwrap();
    let rucksacks: Vec<String> = BufReader::new(file).lines().map(|s| s.unwrap()).collect();
    let rucksacks_common_priority_sum: u32 = rucksacks
        .clone()
        .into_iter()
        .map(|rucksack| {
            let compartment_size = rucksack.len() / 2;
            let compartment1: HashSet<char> =
                HashSet::from_iter(rucksack.chars().take(compartment_size));
            let compartment2: HashSet<char> =
                HashSet::from_iter(rucksack.chars().skip(compartment_size));
            compartment1
                .intersection(&compartment2)
                .next()
                .expect("No common item")
                .to_owned()
        })
        .map(prioritize_item)
        .sum();
    let rucksacks_badge_priority_sum: u32 = rucksacks
        .chunks(3)
        .map(|group_sacks| {
            let sack1: HashSet<char> = HashSet::from_iter(group_sacks.get(0).unwrap().chars());
            let sack2: HashSet<char> = HashSet::from_iter(group_sacks.get(1).unwrap().chars());
            let sack3: HashSet<char> = HashSet::from_iter(group_sacks.get(2).unwrap().chars());
            sack1
                .iter()
                .filter(|c| sack2.contains(c))
                .find(|c| sack3.contains(c))
                .expect("No common item")
                .to_owned()
        })
        .map(prioritize_item)
        .sum();
    println!("Sum of priorities: {rucksacks_common_priority_sum}");
    println!("Sum of badge priorities: {rucksacks_badge_priority_sum}");
}
