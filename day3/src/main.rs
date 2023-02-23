use itertools::Itertools;

fn priority(c: u8) -> Option<isize> {
    match c as char {
        'a'..='z' => Some((c - 96) as isize),
        'A'..='Z' => Some((c - 38) as isize),
        _ => None,
    }
}

fn part1(input: &str) {
    assert!(input.is_ascii());

    let sizes = input.lines().map(str::len);
    let lines = input.lines().map(str::as_bytes);

    let score = lines
        .zip(sizes)
        .map(|(line, size)| line.split_at(size / 2))
        .fold(0isize, |score, (left, right)| {
            let mut bitvec = 0u128;
            left.iter().for_each(|x| bitvec |= 1 << x);
            score
                + right
                    .iter()
                    .find(|x| (bitvec & (1 << *x)) > 0)
                    .copied()
                    .and_then(priority)
                    .unwrap()
        });

    println!("Score: {}", score);
}

fn part2(input: &str) {
    assert!(input.is_ascii());

    let sum: isize = input
        .lines()
        .map(str::as_bytes)
        .map(|sack| {
            let mut bitvec = 0u128;
            sack.iter().for_each(|x| bitvec |= 1 << x);
            bitvec
        })
        .batching(|x| x.take(3).reduce(|accum, bitvec| accum & bitvec))
        .map(|x| priority(x.trailing_zeros() as u8))
        .flatten()
        .sum();

    println!("Sum: {}", sum)
}

fn main() {
    let input = include_str!("input.txt");
    // let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
    // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    // PmmdzqPrVvPwwTWBwg
    // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    // ttgJtRGJQctTZtZT
    // CrZsJsPPZsGzwwsLwLmpwMDw"#;

    part1(input);
    part2(input);
}
