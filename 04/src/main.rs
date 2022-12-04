use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2022-04")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

type Range = (u32, u32);

fn parse_range(range: &String) -> Range {
    let components: Vec<u32> = range
        .split("-")
        .map(|d| d.parse::<u32>().unwrap())
        .collect();
    (
        components.get(0).unwrap().to_owned(),
        components.get(1).unwrap().to_owned(),
    )
}

fn range_encompassed(first: &Range, second: &Range) -> bool {
    (first.0 <= second.0 && first.1 >= second.1) || (second.0 <= first.0 && second.1 >= first.1)
}

fn ranges_overlap(first: &Range, second: &Range) -> bool {
    (first.0 <= second.0 && first.1 >= second.0) || (second.0 <= first.0 && second.1 >= first.0)
}

fn main() {
    let opt = Opt::from_args();
    let file = File::open(&opt.input).unwrap();
    let assignments: Vec<(Range, Range)> = BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .map(|l| l.split(",").map(String::from).collect::<Vec<String>>())
        .map(|p| p.iter().map(parse_range).collect::<Vec<Range>>())
        .map(|p| (p.get(0).unwrap().to_owned(), p.get(1).unwrap().to_owned()))
        .collect();
    let encompassed_count = assignments
        .iter()
        .filter(|(first, second)| range_encompassed(first, second))
        .count();
    let overlapping_count = assignments
        .iter()
        .filter(|(first, second)| ranges_overlap(first, second))
        .count();
    println!("Total encompassed assignments: {encompassed_count}");
    println!("Total overlapping assignments: {overlapping_count}");
}
