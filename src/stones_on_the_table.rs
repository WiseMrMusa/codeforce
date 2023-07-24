use std::io;

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error reading line");
    let n: u8 = n.trim().parse().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Error reading line");
    let s: Vec<char> = s.trim().chars().collect();
    let mut hold: char = s[0];
    let mut res: u8 = 0;
    for i in 0..n{
        if s[i as usize] == hold{
            res += 1;
        }
        hold = s[i as usize];
    }
    println!("{}",res - 1);
}