use std::str::SplitAsciiWhitespace;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn from_char(c: char) -> Option<Self> {
        use Outcome::*;
        match c {
            'X' => Some(Lose),
            'Y' => Some(Draw),
            'Z' => Some(Win),
            _ => None,
        }
    }

    fn score(self) -> isize {
        use Outcome::*;
        match self {
            Lose => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_char(c: char) -> Option<Self> {
        use Shape::*;
        match c {
            'A' | 'X' => Some(Rock),
            'B' | 'Y' => Some(Paper),
            'C' | 'Z' => Some(Scissors),
            _ => None,
        }
    }

    const fn beats(self) -> Shape {
        use Shape::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    const fn loses(self) -> Shape {
        use Shape::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn score_shape(self) -> isize {
        use Shape::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn score_round(self, opponent: Self) -> isize {
        use Shape::*;
        match (self, opponent) {
            (Rock, Paper) => Outcome::Lose.score(),
            (Paper, Rock) => Outcome::Win.score(),
            (Rock, Scissors) => Outcome::Win.score(),
            (Scissors, Rock) => Outcome::Lose.score(),
            (Paper, Scissors) => Outcome::Lose.score(),
            (Scissors, Paper) => Outcome::Win.score(),
            (x, y) if (x == y) => Outcome::Draw.score(),
            _ => unreachable!(),
        }
    }

    pub fn score(self, opponent: Self) -> isize {
        self.score_round(opponent) + self.score_shape()
    }
}

fn predict_move(opponent_move: Shape, desire: Outcome) -> Shape {
    use Outcome::*;
    match (opponent_move, desire) {
        (x, Lose) => x.beats(),
        (x, Win) => x.loses(),
        (x, Draw) => x,
    }
}

fn part1(input: &str) {
    let score = input
        .lines()
        .map(|x| x.split_once(' '))
        .flatten()
        .map(|(l, r)| {
            l.chars()
                .next()
                .and_then(Shape::from_char)
                .zip(r.chars().next().and_then(Shape::from_char))
        })
        .flatten()
        .fold(0, |total, (opponent, me)| total + me.score(opponent));

    println!("Score: {:?}", score);
}

fn part2(input: &str) {
    let score = input
        .lines()
        .map(|x| x.split_once(' '))
        .flatten()
        .map(|(l, r)| {
            l.chars()
                .next()
                .and_then(Shape::from_char)
                .zip(r.chars().next().and_then(Outcome::from_char))
        })
        .flatten()
        .map(|(theirs, desire)| (theirs, predict_move(theirs, desire)))
        .fold(0, |total, (opponent, me)| total + me.score(opponent));

    println!("Score: {:?}", score);
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}
