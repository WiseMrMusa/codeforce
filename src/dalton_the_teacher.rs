use std::io;



pub fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Error reading line");
    let t: u16 = t.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Error reading line");
        let n: u32 = n.trim().parse().unwrap();
        let mut p = String::new();
        io::stdin().read_line(&mut p).expect("Error reading line");
        let p: Vec<u32> = p
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let mut inc = 0;
        for i in 0..n {
            if i as u32 + 1 == p[i as usize] {
                inc += 1;
            }
        }
        let mut res = inc / 2;
        if inc % 2 != 0 {
            res += 1;
        }
        println!("{}", res);
    }
}
