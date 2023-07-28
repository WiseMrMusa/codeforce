use std::io;

pub fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Error reading line");
    let t: u16 = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut nmkh = String::new();
        io::stdin().read_line(&mut nmkh).expect("Error reading line");
        let nmkh: Vec<u32> = nmkh
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Error reading line");
        let n: Vec<u32> = n
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut res: u8 = 0;
        for h in n {
            let w: u32 = if nmkh[3] > h { nmkh[3] - h } else { h - nmkh[3] };
            if w % nmkh[2] == 0 && w / nmkh[2] < nmkh[1] && w / nmkh[2] > 0 {
                res += 1;
            }
        }
        println!("{}",res);
    }
}
