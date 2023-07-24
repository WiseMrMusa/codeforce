use std::io;

pub fn main() {
    let mut nk = String::new();
    io::stdin().read_line(&mut nk).expect("Error reading line");
    let nk: Vec<u32> = nk
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut res = nk[0];
    for _ in 0..nk[1]{
        res = subtract_one(&res)
    }    
    println!("{}", res);
}


fn subtract_one(n:&u32) -> u32 {
    if n%10 == 0 {
        n / 10
    } else {
        n - 1
    }
}