mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use day1::day1;
use day2::day2;
use day3::day3;
use day4::day4;
use day5::day5;

use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::LazyLock;
use std::time::Instant;

struct Paths {
    day1: &'static OsStr,
    day2: &'static OsStr,
    day3: &'static OsStr,
    day4: &'static OsStr,
    day5: &'static OsStr,
    /*day6: &'static OsStr,
    day7: &'static OsStr,
    day8: &'static OsStr,
    day9: &'static OsStr,*/
}

static PATHS: LazyLock<Paths> = LazyLock::new(|| Paths {
    day1: OsStr::new(r"data/day1/input.txt"),
    day2: OsStr::new(r"data/day2/input.txt"),
    day3: OsStr::new("data/day3/input.txt"),
    day4: OsStr::new(r"data/day4/input.txt"),
    day5: OsStr::new(r"data/day5/input.txt"),
    /*day6: OsStr::new(r"data/day6/input.txt"),
    day7: OsStr::new(r"data/day7/input.txt"),
    day8: OsStr::new(r"data/day8/input.txt"),
    day9: OsStr::new(r"data/day9/input.txt"),*/
});

fn main() {
    //day 1
    let start = Instant::now();
    let input = get_input(PATHS.day1);
    let lines = get_lines(&input);
    let res = day1(&lines);
    let duration = start.elapsed();
    println!("Day 1 Result: {:?}, Duration: {:?}", res, duration);

    //day 2 (Currently not correct)
    let start = Instant::now();
    let input = get_input(PATHS.day2);
    let lines = get_lines(&input);
    let res = day2(&lines);
    let duration = start.elapsed();
    println!("Day 2 Result (currently wrong): {:?}, Duration: {:?}", res, duration);

    //day 3
    let start = Instant::now();
    let input = get_input(PATHS.day3);
    let lines = get_lines(&input);
    let res = day3(&lines);
    let duration = start.elapsed();
    println!("Day 3 Result: {:?}, Duration: {:?}", res, duration);

    //day 4
    let start = Instant::now();
    let input = get_input(PATHS.day4);
    let lines = get_lines(&input);
    let res = day4(&lines);
    let duration = start.elapsed();
    println!("Day 4 Result: {:?}, Duration: {:?}", res, duration);

    //day 5
    let start = Instant::now();
    let input = get_input(PATHS.day5);
    let lines = get_lines(&input);
    let res = day5(&lines);
    let duration = start.elapsed();
    println!("Day 5 Result: {:?}, Duration: {:?}", res, duration);
}

fn get_input(path_string: &OsStr) -> String {
    let mut ret_string = String::new();
    BufReader::new(File::open(path_string).unwrap())
        .read_to_string(&mut ret_string)
        .unwrap();
    ret_string
}

fn get_lines(string: &str) -> Vec<&str> {
    string.trim().lines().collect()
}
