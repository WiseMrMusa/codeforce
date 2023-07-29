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

        let k = nk[1] as usize;
        let mut res: Vec<usize> = vec![0; k];
        let mut count_zeros = 0;

        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("Error reading line");
        a.split_whitespace().enumerate().for_each(|(i, x)| {
            let val = (x.parse::<u32>().unwrap() as u32) % k as u32;
            if val == 0 {
                count_zeros += 1;
                print!("{} ", i + 1);
            } else {
                if res[val as usize] == 0 {
                    res[val as usize] = i + 1;
                }
            }
        });

        for &pos in &res {
            // if pos != 0 {
                print!("{} ", pos);
            // }
        }

        println!();
    }
}
