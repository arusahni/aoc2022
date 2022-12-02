use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2022-01")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let file = File::open(&opt.input).unwrap();
    let (mut party_cals, last_val) = BufReader::new(file)
        .lines()
        .map(|s| s.unwrap().parse::<u32>().ok())
        .fold(
            (Vec::<u32>::new(), 0),
            |(mut party_cals, current_cals), calories| match calories {
                Some(item_cals) => (party_cals, current_cals + item_cals),
                None => {
                    party_cals.push(current_cals);
                    (party_cals, 0)
                }
            },
        );
    party_cals.push(last_val);
    party_cals.sort_by(|a, b| b.cmp(a));
    println!("Max party cals: {}", party_cals.first().unwrap());
    println!("Top 3 cals total: {}", party_cals.get(0..3).unwrap().iter().sum::<u32>());
}
