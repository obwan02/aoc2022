#![feature(test)]

trait Range {
    fn from_string(range: &str) -> Option<Self>
    where
        Self: Sized,
    {
        let (begin, end) = range.split_once('-')?;
        let begin: u8 = begin.parse().ok()?;
        let end: u8 = end.parse().ok()?;

        Some(Self::from_range(begin, end))
    }

    fn from_range(begin: u8, end: u8) -> Self;
    fn full_overlap(self, other: Self) -> bool;
    fn any_overlap(self, other: Self) -> bool;
}

#[derive(Copy, Clone, Debug)]
struct BitMask(u128);

impl Range for BitMask {
    fn from_range(begin: u8, end: u8) -> Self {
        let (shl, shr) = (begin, 127 - end);
        BitMask((u128::MAX.wrapping_shl(shl as u32)) & (u128::MAX.wrapping_shr(shr as u32)))
    }

    fn full_overlap(self: BitMask, b: BitMask) -> bool {
        let overlap = self.0 & b.0;
        let count = overlap.count_ones();
        count == self.0.count_ones() || count == b.0.count_ones()
    }

    fn any_overlap(self, other: Self) -> bool {
        (self.0 & other.0) > 0
    }
}

#[derive(Copy, Clone, Debug)]
struct NaiveRange(u8, u8);

impl Range for NaiveRange {
    fn from_range(begin: u8, end: u8) -> Self {
        NaiveRange(begin, end)
    }

    fn full_overlap(self, other: Self) -> bool {
        return (self.0 >= other.0 && self.1 <= other.1)
            || (self.0 <= other.0 && self.1 >= other.1);
    }

    fn any_overlap(self, other: Self) -> bool {
        return self.0 <= other.1 && self.1 >= other.0;
    }
}

fn part1<T: Range + Copy + Clone>(input: &str) -> usize {
    input
        .lines()
        .flat_map(|x| x.split_once(','))
        .flat_map(|(left, right)| T::from_string(left).zip(T::from_string(right)))
        .filter(|(left, right)| left.full_overlap(*right))
        .count()
}

fn part2<T: Range + Copy>(input: &str) -> usize {
    input
        .lines()
        .flat_map(|x| x.split_once(','))
        .flat_map(|(left, right)| T::from_string(left).zip(T::from_string(right)))
        .filter(|(left, right)| left.any_overlap(*right))
        .count()
}

fn main() {
    let input = include_str!("input.txt");
    let p1 = part1::<BitMask>(input);
    let p2 = part2::<BitMask>(input);

    println!("P1: {}, P2: {}", p1, p2);
}

#[cfg(test)]
mod tests {

    extern crate test;
    use super::*;
    use test::Bencher;

    const INPUT: &'static str = include_str!("input.txt");

    #[test]
    fn bitmask_eq_naive_p1() {
        assert_eq!(part1::<BitMask>(INPUT), part1::<NaiveRange>(INPUT))
    }

    #[test]
    fn bitmask_eq_naive_p2() {
        assert_eq!(part2::<BitMask>(INPUT), part2::<NaiveRange>(INPUT))
    }

    #[bench]
    fn part1_bitmask(bench: &mut Bencher) {
        bench.iter(|| part1::<BitMask>(INPUT));
    }

    #[bench]
    fn part1_naive(bench: &mut Bencher) {
        bench.iter(|| part1::<NaiveRange>(INPUT));
    }

    #[bench]
    fn part2_bitmask(bench: &mut Bencher) {
        bench.iter(|| part2::<BitMask>(INPUT));
    }

    #[bench]
    fn part2_naive(bench: &mut Bencher) {
        bench.iter(|| part2::<NaiveRange>(INPUT));
    }
}
