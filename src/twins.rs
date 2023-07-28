use std::io;

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error reading line");
    let n: u8 = n.trim().parse().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Error reading line");
    let mut x: Vec<u32> = x
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    x.sort_by(|x, y| y.cmp(&x));

    let mut sum: u32 = 0;
    for y in &x {
        sum += y;
    }

    let mut xum: u32 = x[0 as usize];
    if xum > sum / 2 {
        println!("{}",1);
    } else {
        for i in 1..n {
            xum += x[i as usize];
            if xum > sum / 2 {
                println!("{}", i + 1);
                break;
            }
        }
    }
}
