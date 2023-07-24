use std::io;

pub fn main(){
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Error reading line");
    let s: &str = s.trim();
    let mut upc: u8 = 0;
    for c in s.chars() {
        if c.is_uppercase() {
            upc += 1;
        }
    }
    if usize::from(upc) > s.len()/2 {
        println!("{}", s.to_uppercase());
    } else {
        println!("{}", s.to_lowercase());
    }
}