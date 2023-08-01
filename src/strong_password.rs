use std::io::{self, BufRead};

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let s: Vec<u32> = lines
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let n = s.len();
        let m: usize = lines.next().unwrap().trim().parse().unwrap();
        let l: Vec<u32> = lines
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let r: Vec<u32> = lines
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let mut mx = 0;
        for i in 0..m {
            let li = l[i] as usize;
            let ri = r[i] as usize;
            let mut nmx = mx;
            for c in li..=ri {
                let mut cur = mx;
                while cur < n && s[cur] != c as u32 {
                    cur += 1;
                }
                nmx = nmx.max(cur);
            }
            mx = nmx + 1;
        }

        println!("{}", if mx > n { "YES" } else { "NO" });
    }
}