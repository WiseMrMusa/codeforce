use std::io;

pub fn main() {
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Error reading line");
    let mut name: Vec<char> = name
        .trim()
        .chars()
        .collect();
    name.sort();
    let mut temp: char = name[0];
    let mut res: u8 = 1;
    for i in 0..name.len(){
        if name[i] != temp {
            res += 1;
        }
        temp = name[i]
    }

    if res%2 == 0 {
        println!("CHAT WITH HER!");
    } else {
        println!("IGNORE HIM!");
    }
}
