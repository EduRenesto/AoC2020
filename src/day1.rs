use std::io::{ self, BufRead };

// Completely trivial solution.
// The next ones will be better thought out. This one was made in a hurry
// because I have tons of assignments due soon

pub fn run_part1() {
    let stdin = io::stdin();

    let data = stdin.lock()
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    for i in 0..data.len() {
        for j in &data[i+1..] {
            let i = data[i];

            if i + j == 2020 {
                println!("{} * {} = {}", i, j, i * j);
                return;
            }
        }
    }
}

pub fn run_part2() {
    let stdin = io::stdin();

    let data = stdin.lock()
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    for i in 0..data.len() {
        for j in &data[i+1..] {
            for k in &data[i+2..] {
                let i = data[i];

                if i + j + k == 2020 {
                    println!("{} * {} * {} = {}", i, j, k, i * j * k);
                    return;
                }
            }
        }
    }
}
