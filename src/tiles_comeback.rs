use std::io;

pub fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Error reading line");
    let t: u16 = t.trim().parse().unwrap();
    for _ in 0..t {
        let mut nk = String::new();
        io::stdin().read_line(&mut nk).expect("Error reading line");
        let nk: Vec<u32> = nk
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut c = String::new();
        io::stdin().read_line(&mut c).expect("Error reading line");
        let c: Vec<u32> = c
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if c[0 as usize] == c[nk[0] as usize - 1 ] {
            let mut count = 0;
            for i in c.iter() {
                if *i == c[0] {
                    count += 1;
                }
            }
            if count >= nk[1] {
                println!("YES");
            } else {
                println!("NO");
            }
        } else {
            let mut beg = 0;
            let mut end = 0;
            for i in c.iter() {
                if beg < nk[1] {
                    if *i == c[0] {
                        beg += 1;
                    }
                } else {
                    if *i == c[nk[0] as usize -1] {
                        end += 1;
                    }
                }
            }
            if end >= nk[1] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
