use std::io;

pub fn main() {
    let mut sit = String::new();
    io::stdin().read_line(&mut sit).expect("Error reading line");
    let sit = sit.trim();
    let mut temp: char = sit[0..1].parse().unwrap();
    let mut ehm: u8 = 0;
    for c in sit.chars() {
        if ehm == 7 {
            break;
        }

        if c == temp {
            ehm += 1;
            // println!("{}",ehm);
        } else {
            temp = c;
            ehm = 1;
            // println!("{}",ehm);
        }
    }
    if ehm == 7 {
        println!("YES");
    } else {
        println!("NO");
    }
}
