use std::io::{ self, BufRead };

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let n: usize = lines.next().unwrap().trim().parse().unwrap();
        let mut delta: i64 = 0;
        let mut ans: i64 = 0;
        let mut sum: i64 = 0;
        let mut mx: i64 = 0;

        let xx: Vec<i64> = lines
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        for x in xx {
            sum += x;
            mx = mx.max(sum);

            if sum - mx < delta {
                delta = sum - mx;
                ans = mx;
            }
        }

        println!("{}", ans);
    }
}
