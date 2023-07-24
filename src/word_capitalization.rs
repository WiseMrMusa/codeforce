use std::io;

pub fn main(){
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Error reading line");
    let word: &str = word.trim();
    let wordlen = word.len();
    let mut res = String::new();
    res.push_str(&word[0..1].to_ascii_uppercase());
    res.push_str(&word[1..wordlen]);
    println!("{}",res);
}