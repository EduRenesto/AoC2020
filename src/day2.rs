use std::{ io, io::BufRead };
use std::borrow::Cow;

use rayon::prelude::*;

// Part 1 is beautiful-ish, part 2 could use some refactoring.
// Using rayon might be a bit overkill, but I did anyway

struct Matcher {
    target: char,
    min: usize,
    max: usize,
    count: usize,
}

impl Matcher {
    pub fn new(target: char, min: usize, max: usize) -> Matcher {
        Matcher {
            target,
            min,
            max,
            count: 0,
        }
    }

    fn step<'a>(&mut self, input: &'a str) -> Option<&'a str> {
        if input.chars().next()? == self.target {
            self.count += 1;
        }

        Some(&input[1..])
    }

    fn run<'a>(&mut self, input: &'a str) {
        if input.len() < self.min {
            return;
        } else {
            let mut input = input;
            while let Some(cur) = self.step(input) {
                input = cur;
            }
        }
    }

    pub fn accepts<'a>(&self) -> bool {
        self.count >= self.min && self.count <= self.max
    }
}

fn parse_input(input: String) -> (Matcher, String) {
    let mut chars = input.chars();

    let min = chars
        .by_ref()
        .take_while(|&c| c != '-')
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let max = chars
        .by_ref()
        .take_while(|&c| c != ' ')
        .collect::<Cow<str>>()
        .parse::<usize>()
        .unwrap();

    let target = chars.next().unwrap();

    chars.next().unwrap();
    chars.next().unwrap();

    let password = chars.collect::<String>();

    let matcher = Matcher::new(target, min, max);

    (matcher, password)
}

pub fn run_part1() {
    let stdin = io::stdin();

    let mut data = stdin.lock()
        .lines()
        .map(|line| {
            parse_input(line.unwrap())
        })
        .collect::<Vec<_>>();

    let res = data.par_iter_mut()
        .update(|(matcher, password)| {
            matcher.run(&password)
        })
        .filter(|(matcher, _)| matcher.accepts())
        .count();

    println!("{}", res);
}

fn parse_input_part2(input: String) -> (usize, usize, char, String) {
    let mut chars = input.chars();

    let left = chars
        .by_ref()
        .take_while(|&c| c != '-')
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let right = chars
        .by_ref()
        .take_while(|&c| c != ' ')
        .collect::<Cow<str>>()
        .parse::<usize>()
        .unwrap();

    let target = chars.next().unwrap();

    chars.next().unwrap();
    chars.next().unwrap();

    let password = chars.collect::<String>();

    (left, right, target, password)
}

fn matches_part_2(left: usize, right: usize, target: u8, input: &str) -> bool {
    if input.len() < right - 1 {
        false
    } else {
        let chars = input.as_bytes();
        (chars[left - 1] == target) ^ (chars[right - 1] == target)
    }
}

pub fn run_part2() {
    let stdin = io::stdin();

    let mut data = stdin.lock()
        .lines()
        .map(|line| {
            parse_input_part2(line.unwrap())
        })
        .collect::<Vec<_>>();

    let res = data.par_iter_mut()
        .filter(|(left, right, target, password)| matches_part_2(*left, *right, *target as u8, password))
        .count();

    println!("{}", res);
}

mod test {
    use crate::day2::{ parse_input, parse_input_part2, matches_part_2 };
    #[test]
    fn test_part1() {
        for (src, min, max, target, pass, accepts) in vec![
            ("1-3 a: abcde", 1, 3, 'a', "abcde", true),
            ("1-3 b: cdefg", 1, 3, 'b', "cdefg", false),
            ("2-9 c: ccccccccc", 2, 9, 'c', "ccccccccc", true),
        ] {
            let (mut matcher, npass) = parse_input(String::from(src));
            assert_eq!(matcher.min, min);
            assert_eq!(matcher.max, max);
            assert_eq!(matcher.target, target);
            assert_eq!(npass, pass);
            matcher.run(&npass);
            assert_eq!(matcher.accepts(), accepts);
        }
    }
    #[test]
    fn test_part2() {
        for (src, left, right, target, pass, accepts) in vec![
            ("1-3 a: abcde", 1, 3, 'a', "abcde", true),
            ("1-3 b: cdefg", 1, 3, 'b', "cdefg", false),
            ("2-9 c: ccccccccc", 2, 9, 'c', "ccccccccc", false),
        ] {
            let (nleft, nright, ntarget, npass) = parse_input_part2(String::from(src));
            assert_eq!(left, nleft);
            assert_eq!(right, nright);
            assert_eq!(target, ntarget);
            assert_eq!(pass, npass);
            assert_eq!(accepts, matches_part_2(left, right, target as u8, &npass));
        }
    }
}
