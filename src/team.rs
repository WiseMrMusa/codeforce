use std::io;

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error reading line");
    let n: u16 = n.trim().parse().unwrap();

    let mut res: u16 = 0;

    for _ in 0..n {
        let mut u = String::new();
        io::stdin().read_line(&mut u).expect("Error reading line");
        let u: Vec<u16> = u
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        res += team(u);
    }
    println!("{}", res);
}

fn team(a: Vec<u16>) -> u16 {
    let sum: u16 = a[0] + a[1] + a[2];
    if sum < 2 {
        0
    } else {
        1
    }
}