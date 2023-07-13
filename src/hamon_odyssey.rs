//https://codeforces.com/contest/1847/problem/B

use std::io;

pub fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Error reading line");
    let t: u16 = t.trim().parse().unwrap();
    for _ in 0..t {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Error reading line");
        let n: u32 = n.trim().parse().unwrap();
        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("Error reading line");
        let a: Vec<u32> = a
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        maximum_group(n,a);
    }
}
fn maximum_group( n: u32, a: Vec<u32>){
    let mut result: u32  = 0;
    let mut strength: u32 = a[0];
    for i in 0..n {
        let val = strength & (a[i as usize]);
        if val == 0 {
            result += 1;
            if i != n - 1 {
                strength = a[(i+1) as usize];
            }
        } else {
            strength = val;
        }
    }
    if result == 0 {
        result = 1;
    }
    println!("{}", result);
}