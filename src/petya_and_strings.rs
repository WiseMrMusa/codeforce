use std::io;

pub fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Errror reading line");
    let a: &str = a.trim();
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Error reading line");
    let b: &str = b.trim();

    let res : i8 = if a.to_lowercase() > b.to_lowercase() {
        1
    } else if a.to_lowercase() < b.to_lowercase() {
        - 1
    } else { 0 };

    println!("{}",res);
}