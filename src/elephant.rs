use std::io;

pub fn main(){
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Error reading line");
    let x: u32 = x.trim().parse().unwrap();
    let mut res: u32 = x/5;
    if x%5 != 0 {
        res += 1;
    }
    println!("{}",res);
}