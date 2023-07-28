use std::io;

pub fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Error reading line");
    let t: u16 = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut bch = String::new();
        io::stdin().read_line(&mut bch).expect("Error reading line");
        let bch: Vec<u8> = bch
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if bch[1] + bch[2] < bch[0] {
            let res: u8 = 1+ 2*(bch[1]+bch[2]);
            println!("{}", res);
        } else {
            let res: u8 = 2*(bch[0]) - 1; 
            println!("{}", res);
        }
    }
}
