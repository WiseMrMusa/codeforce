use std::io;

pub fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");
    let mut input: Vec<char> = input.trim().split("+").map(|x| x.parse().unwrap()).collect();
    input.sort();
    let mut res = String::from(input[0]);
    if input.len() > 1 {
        for i in 1..input.len() {
            res.push('+');
            res.push(input[i]);
        }
    }
    // res.push(input[1]);
    println!("{}", res);
}