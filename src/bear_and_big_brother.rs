use std::io;

pub fn main() {
    let mut ab = String::new();
    io::stdin().read_line(&mut ab).expect("Error reading line");
    let ab: Vec<u32> = ab
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut a = ab[0];
    let mut b = ab[1];
    let mut res = 0;
    for i in 1..7 {
        a *= 3;
        b *= 2;
        if a > b {
            res = i;
            break;
        }
    }
    println!("{}", res);
}
