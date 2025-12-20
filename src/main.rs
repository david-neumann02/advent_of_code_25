mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use day1::day1;
use day2::day2;
use day3::day3;
use day4::day4;
use day5::day5;
use day6::day6;

use std::ffi::OsStr;
use std::fmt::Debug;
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
    day6: &'static OsStr,
    /*day7: &'static OsStr,
    day8: &'static OsStr,
    day9: &'static OsStr,*/
}

static PATHS: LazyLock<Paths> = LazyLock::new(|| Paths {
    day1: OsStr::new(r"data/day1/input.txt"),
    day2: OsStr::new(r"data/day2/input.txt"),
    day3: OsStr::new("data/day3/input.txt"),
    day4: OsStr::new(r"data/day4/input.txt"),
    day5: OsStr::new(r"data/day5/input.txt"),
    day6: OsStr::new(r"data/day6/input.txt"),
    /*day7: OsStr::new(r"data/day7/input.txt"),
    day8: OsStr::new(r"data/day8/input.txt"),
    day9: OsStr::new(r"data/day9/input.txt"),*/
});

fn main() {
    //day 1
    run_day(DaySolution::new(PATHS.day1, 1, day1));

    //day 2 (Currently not correct)
    run_day(DaySolution::new(PATHS.day2, 2, day2));

    //day 3
    run_day(DaySolution::new(PATHS.day3, 3, day3));

    //day 4
    run_day(DaySolution::new(PATHS.day4, 4, day4));

    //day 5
    run_day(DaySolution::new(PATHS.day5, 5, day5));

    //day 6
    run_day(DaySolution::new(PATHS.day6, 6, day6));
}

struct DaySolution<T: Sized + Debug>{
    input_path: &'static OsStr,
    day_number: u32,
    solve_function: fn(&Vec<&str>) -> T
}

impl <T: Sized + Debug> DaySolution<T> {
    fn new(input_path: &'static OsStr, day_number: u32, solve_function: fn(&Vec<&str>) -> T) -> DaySolution<T> {
        DaySolution { input_path, day_number, solve_function }
    }
}

fn run_day<T: Debug + Sized>(day_solution: DaySolution<T>) {
    let start = Instant::now();
    let input = get_input(day_solution.input_path);
    let lines = get_lines(&input);
    let res = (day_solution.solve_function)(&lines);
    let duration = start.elapsed();
    println!("Day {} Result: {:?}, Duration: {:?}", day_solution.day_number, res, duration)
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
