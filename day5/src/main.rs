use parse::{Crate, Parse};

mod parse;

fn part1(mut parse: Parse) {
    for mv in parse.moves {
        let from_index = mv.from - 1;
        let to_index = mv.to - 1;
        // SAFETY:
        // The only unsafe operation in this block is dereferencing to *mut pointer.
        // However, we know this pointer must be valid as it comes from a `Vec`. We also
        // know that the &mut references won't alias because we assert that mv.from != mv.to
        let (from, to) = unsafe {
            assert!(mv.from != mv.to);
            (
                &mut *(parse.yard_cols.get_mut(from_index).unwrap() as *mut Vec<Crate>),
                &mut *(parse.yard_cols.get_mut(to_index).unwrap() as *mut Vec<Crate>),
            )
        };

        let drain_range = (from.len().saturating_sub(mv.count))..from.len();
        to.extend(from.drain(drain_range).rev());
    }

    print!("Result for Part 1: ");
    let out: String = parse
        .yard_cols
        .iter()
        .map(|col| col.last().map(|krate| krate.0 as char).unwrap_or('-'))
        .collect();
    println!("{}", out);
}

fn part2(mut parse: Parse) {
    for mv in parse.moves {
        let from_index = mv.from - 1;
        let to_index = mv.to - 1;
        // SAFETY:
        // The only unsafe operation in this block is dereferencing to *mut pointer.
        // However, we know this pointer must be valid as it comes from a `Vec`. We also
        // know that the &mut references won't alias because we assert that mv.from != mv.to
        let (from, to) = unsafe {
            assert!(mv.from != mv.to);
            (
                &mut *(parse.yard_cols.get_mut(from_index).unwrap() as *mut Vec<Crate>),
                &mut *(parse.yard_cols.get_mut(to_index).unwrap() as *mut Vec<Crate>),
            )
        };

        let drain_range = (from.len().saturating_sub(mv.count))..from.len();
        to.extend(from.drain(drain_range));
    }

    print!("Result for Part 2: ");
    let out: String = parse
        .yard_cols
        .iter()
        .map(|col| col.last().map(|krate| krate.0 as char).unwrap_or('-'))
        .collect();
    println!("{}", out);
}

fn main() {
    let parse_crate_test = include_str!("input.txt");

    let parse = match parse::parse(parse_crate_test) {
        Ok(x) => x,
        Err(err) => {
            println!("error: {}", err);
            return;
        }
    };

    part1(parse.clone());
    part2(parse);
}
