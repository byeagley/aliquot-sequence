use num::integer::div_rem;
use std::collections::HashSet;
use std::env;
use std::str::FromStr;

fn get_proper_divisors(number: u64) -> HashSet<u64> {
    let mut factors: HashSet<u64> = HashSet::new();

    if number == 1 {
        return factors;
    }

    factors.insert(1);
    for n in 2..(number + 1 / 2) {
        if let (div, 0) = div_rem(number, n) {
            factors.insert(div);
            factors.insert(n);
        }
    }
    factors
}

struct Aliquot {
    curr: u64,
}

impl Iterator for Aliquot {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.curr {
            0 => None,
            _ => Some(self.curr),
        };

        self.curr = get_proper_divisors(self.curr).into_iter().sum();

        ret
    }
}

fn aliquot(number: u64) -> Aliquot {
    Aliquot { curr: number }
}

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() > 1 {
        eprintln!("Usage: aliquot NUMBER");
        std::process::exit(1);
    }

    let d = numbers[0];

    let mut sequence: HashSet<u64> = HashSet::new();

    for i in aliquot(d).take(10) {
        let res = sequence.insert(i);
        if res == false {
            println!("loop detected, starting at {}", i);
            break;
        }
        println!("> {}", i);
    }
}
