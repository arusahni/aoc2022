use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2022-02")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win = 0,
    Lose = 3,
    Tie = 6,
}

impl Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", *self as u32))
    }
}

impl Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", *self as u32))
    }
}

fn decrypt_hand(encrypted: &str) -> Play {
    match encrypted {
        "A" | "X" => Play::Rock,
        "B" | "Y" => Play::Paper,
        "C" | "Z" => Play::Scissors,
        _ => panic!("Unrecognized symbol: {encrypted}"),
    }
}

fn decrypt_outcome(encrypted: &str) -> Outcome {
    match encrypted {
        "X" => Outcome::Lose,
        "Y" => Outcome::Tie,
        "Z" => Outcome::Win,
        _ => panic!("Unrecognized symbol: {encrypted}"),
    }
}

fn throw(theirs: &Play, mine: &Play) -> Outcome {
    println!("{theirs}, {mine}");
    if *theirs as u32 == *mine as u32 {
        Outcome::Tie
    } else if get_winning_play(theirs) as u32 == *mine as u32 {
        Outcome::Win
    } else {
        Outcome::Lose
    }
}

fn get_winning_play(play: &Play) -> Play {
    match play {
        Play::Rock => Play::Paper,
        Play::Paper => Play::Scissors,
        Play::Scissors => Play::Rock,
    }
}

fn select_play(their_play: &Play, desired_outcome: &Outcome) -> Play {
    match *desired_outcome {
        Outcome::Tie => their_play.to_owned(),
        Outcome::Win => get_winning_play(their_play),
        Outcome::Lose => match get_winning_play(their_play) {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        },
    }
}

fn main() {
    let opt = Opt::from_args();
    let mut file = File::open(&opt.input).unwrap();
    let strategy_1: u32 = BufReader::new(file)
        .lines()
        .map(|l| {
            let decrypted: Vec<Play> = l.unwrap().split(' ').map(decrypt_hand).take(2).collect();
            throw(decrypted.get(0).unwrap(), decrypted.get(1).unwrap()) as u32
                + *decrypted.get(1).unwrap() as u32
        })
        .sum();
    file = File::open(&opt.input).unwrap();
    let strategy_2: u32 = BufReader::new(file)
        .lines()
        .map(|l| {
            let hand: Vec<String> = l.unwrap().split(' ').map(String::from).collect();
            (
                decrypt_hand(hand.get(0).unwrap()),
                decrypt_outcome(hand.get(1).unwrap()),
            )
        })
        .map(|(their_play, desired_outcome)| {
            select_play(&their_play, &desired_outcome) as u32 + desired_outcome as u32
        })
        .inspect(|s| println!("{s}"))
        .sum();
    println!("Total score (hands): {strategy_1}");
    println!("Total score (outcomes): {strategy_2}");
}
