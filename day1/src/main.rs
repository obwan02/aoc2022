use itertools::Itertools;

fn part1(input: &str) {
    let (_, max) = input.lines().fold((0, 0), |(curr, max), elem| {
        if !elem.is_empty() {
            return (curr + elem.parse::<isize>().unwrap(), max);
        }

        (0, isize::max(curr, max))
    });

    println!("Maximum Calories: {}", max);
}

#[allow(dead_code)]
fn part2(input: &str) {
    let elves: isize = input
        .lines()
        .map(|x| x.parse::<isize>().ok())
        .coalesce(|curr, next| match (curr, next) {
            (Some(x), Some(y)) => Ok(Some(x + y)),
            (Some(_), None) => Err((curr, next)),
            (None, Some(y)) => Ok(Some(y)),
            (None, None) => Ok(None),
        })
        .flatten()
        .map(std::cmp::Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum();

    println!(
        "The greediest elves have {} Calories in total. Get them!",
        elves
    )
}

fn main() {
    let input = include_str!("input.txt");
    part1(&input);
    part2(&input);
}
