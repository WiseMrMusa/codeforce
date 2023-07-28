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
        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("Error reading line");
        let a: Vec<u32> = a
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap() as u32 % nk[1])
            .collect();
        let mut res: Vec<String> = vec!["".to_string(); nk[1] as usize];
        let mut re: String = String::new();
        for i in 0..a.len() {
            if a[i] == 0 {
                re.push_str((i + 1).to_string().as_str());
                re.push(' ');
            } else {
                res[nk[1] as usize - a[i] as usize].push_str((i + 1).to_string().as_str());
                res[nk[1] as usize - a[i] as usize].push(' ');
            }
        }
        let result_string: String = re.to_string() + &res.join("");
        println!("{}", result_string);
    }
}
