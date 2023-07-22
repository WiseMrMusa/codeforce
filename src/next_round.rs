use std::io;

pub fn main() {
    let mut nk = String::new();
    io::stdin().read_line(&mut nk).expect("Error reading line");
    let nk: Vec<u8> = nk
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Error reading line");
    let a: Vec<u8> = a
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let cutoff: u8 = a[nk[1] as usize - 1];
    let mut res: u8 = if cutoff > 0 { nk[1] } else { 0 };

    if cutoff > 0 {
        for i in nk[1]..nk[0] {
            if a[i as usize] == cutoff {
                res += 1;
            } else { break }
        }
    } else {
        for i in (0..nk[1]).rev() {
            if a[i as usize] > 0 {
                res = i + 1;
                break
            }
        }
    }

    println!("{}", res)
}
