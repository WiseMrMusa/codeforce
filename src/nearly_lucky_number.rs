use std::io;

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error reading line");
    let n: &str = n.trim();
    let mut res: u8 = 0;
    for c in n.chars(){
        if c == '7' || c == '4' {
            res += 1;
        }
    }
    if res == 7 || res == 4 {
        println!("YES");
    } else {
        println!("NO");
    }
}
