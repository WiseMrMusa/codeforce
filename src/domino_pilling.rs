use std::io;

pub fn main() {
    let mut mn = String::new();
    io::stdin().read_line(&mut mn).expect("Error reading line");
    let mn: Vec<u32> = mn
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}",mn[0]*mn[1]/2);
}
