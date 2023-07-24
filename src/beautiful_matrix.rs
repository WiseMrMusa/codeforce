use std::io;

pub fn main() {
    // let mut i = String::new();
    // io::stdin().read_line(&mut i).expect("Error reading the line");
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    for m in 1..=5 {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Error reading input");
        let n: Vec<u8> = n
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        for q in 1..=5 {
            if n[q - 1] == 1 {
                i = m;
                j = q as u8;
            }
        }
    }

    let i = (3 - i as i8).abs();
    let j = (3 - j as i8).abs();

    println!("{}", i+j);
}
