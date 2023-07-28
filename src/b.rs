use std::io;

fn sort_by_cond(a: &(i32, i32), b: &(i32, i32)) -> bool {
    if a.0 != b.0 {
        a.0 < b.0
    } else {
        a.1 > b.1
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut v: Vec<i32> = Vec::new();

    input.clear();
    io::stdin().read_line(&mut input).expect("Error reading line");
    for a in input.trim().split_whitespace() {
        let num: i32 = a.parse().unwrap() % k;
        v.push(num);
    }

    for (i, &value) in v.iter().enumerate() {
        if value == 0 {
            print!("{} ", i + 1);
        }
    }

    let mut v1: Vec<(i32, i32)> = Vec::new();
    for (i, &value) in v.iter().enumerate() {
        v1.push((value, i as i32 + 1));
    }

    v1.sort_by(|a, b| sort_by_cond(a, b));

    for pair in &v1 {
        if pair.0 != 0 {
            print!("{} ", pair.1);
        }
    }

    println!();
}
