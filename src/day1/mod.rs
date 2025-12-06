use std::ops::{Add, AddAssign};

#[derive(Copy, Clone)]
enum Action {
    Left(u64),
    Right(u64),
}

struct Dial(u64);

impl Dial {
    const fn new() -> Dial {
        Dial(50)
    }

    fn check_for_0(&self) -> bool {
        self.0 == 0
    }
}

impl Add<Action> for Dial {
    type Output = Dial;

    fn add(self, rhs: Action) -> Self::Output {
        match rhs {
            Action::Left(neg) => Dial((100 + self.0 - neg) % 100),
            Action::Right(add) => Dial((self.0 + add) % 100),
        }
    }
}

impl AddAssign<&Action> for Dial {
    fn add_assign(&mut self, rhs: &Action) {
        match rhs {
            Action::Left(neg) => {
                self.0 = (100 + self.0 - neg % 100) % 100;
            }
            Action::Right(add) => {
                self.0 = (self.0 + add) % 100;
            }
        }
    }
}

pub(crate) fn day1(lines: &Vec<&str>) -> (u64, u64) {
    let actions = lines
        .iter()
        .map(|s| {
            let (d, n) = s.split_at(1);
            let n: u64 = n.parse().expect("Couldn't parse number in Day1 Input");
            match d {
                "L" => Action::Left(n),
                "R" => Action::Right(n),
                _ => panic!("Encountered unexpected character in Day1 Input"),
            }
        })
        .collect();
    (part1(&actions), part2(&actions))
}

fn part1(actions: &Vec<Action>) -> u64 {
    let mut dial = Dial::new();
    let mut zero_count: u64 = 0;

    for action in actions {
        dial += action;
        if dial.check_for_0() {
            zero_count += 1;
        }
    }

    zero_count
}

fn part2(actions: &Vec<Action>) -> u64 {
    let mut dial = Dial::new();
    let mut passes_count: u64 = 0;

    for action in actions {
        passes_count += match action {
            Action::Right(pos) => (dial.0 + pos) / 100,
            Action::Left(neg) => {
                if dial.0 <= *neg {
                    (neg - dial.0) / 100 + if dial.0 != 0 { 1 } else { 0 }
                } else {
                    0
                }
            }
        };
        dial += action;
    }

    passes_count
}
