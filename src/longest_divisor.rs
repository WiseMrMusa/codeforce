use std::io;

pub fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Error reading line");
    let t: u16 = t.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Error reading line");
        let n: u64 = n.trim().parse().unwrap();
        println!("{}",find_divisor(n));   
    }
}

fn find_divisor(n: u64) -> u64 {
    let mut max_divisor: u64 = 1;
    for i in 1..=n {
        if n % i != 0 {
            break;
        }
        max_divisor = i;
    }
    max_divisor
}