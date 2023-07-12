// https://codeforces.com/problemset/problem/1847/A

use std::io;

pub fn main() {
    let mut input = String::new();

    // Read the number of test cases
    io::stdin().read_line(&mut input).expect("Error reading line");
    let input: u8 = input.trim().parse().unwrap();

    // Read the test cases
    for _ in 0..input {
        let mut nk = String::new();
        io::stdin().read_line(&mut nk).expect("Error reading line");
        let nk: Vec<u8> = nk
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let n: u8 = nk[0];
        let k: u8 = nk[1];

        let mut suspicion = String::new();
        io::stdin().read_line(&mut suspicion).expect("Error reading line");
        let suspicion: Vec<u16> = suspicion
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        minimum_power(n, k, suspicion);
    }
}

fn minimum_power(n: u8, k: u8, suspicion: Vec<u16>) {

    let mut hold: Vec<u16> = Vec::new();
    for i in 0..suspicion.len() - 1 {
        hold.push( (suspicion[i] as i32 - suspicion[i + 1] as i32).abs() as u16);
    }
    hold.sort();
    let result: u16 = hold.iter().take((n-k) as usize).sum();
    println!("{}", result);
}
