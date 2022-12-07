use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2022-06")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let file = File::open(&opt.input).unwrap();
    let line = BufReader::new(file).lines().next().unwrap().unwrap();
    let mut buffer: VecDeque<char> = VecDeque::new();
    for len in &[4, 14] {
        for (ix, char) in line.char_indices() {
            buffer.push_back(char.to_owned());
            if buffer.len() == len + 1 {
                let chars: HashSet<&char> = HashSet::from_iter(buffer.iter().skip(1));
                if chars.len() == *len {
                    println!(
                        "[{}] Found at {}: {}",
                        len,
                        ix + 1,
                        String::from_iter(buffer.iter().skip(1))
                    );
                    break;
                }
            }
            if buffer.len() > *len {
                buffer.pop_front();
            }
        }
    }
}
