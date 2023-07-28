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
        let mut a: Vec<u32> = a
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if n == 1 {
            println!("YES");
        } else {
            let b = a.clone();
            a.sort();
            let mut res: bool = false;
            for i in 0..n {
                if (b[i as usize] as i32 - a[i as usize] as i32).abs() % 2 != 0 {
                    res = true;
                    println!("NO");
                    break;
                }
            }
            if !res { println!("YES");}
        }
    }
}
