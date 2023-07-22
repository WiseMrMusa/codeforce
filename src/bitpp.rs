// https://codeforces.com/problemset/problem/282/A

use std::io;

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error reading the line");
    let n: u16 = n.trim().parse().unwrap();
    let mut res: i16 = 0;
    for _ in 0..n {
        let mut u = String::new();
        io::stdin().read_line(&mut u).expect("Error reading line");
        let u: &str = u.trim();
        match u {
            "++X" => { res += 1 },
            "X++" => { res += 1 },
            "--X" => { res -= 1 },
            "X--" => { res -= 1 },
            &_ => { res += 0 },
        }
    }
    println!("{}", res);
}