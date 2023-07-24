use std::io;

pub fn main(){
    let mut knw = String::new();
    io::stdin().read_line(&mut knw).expect("Error reading line");
    let knw: Vec<u32> = knw.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let k: u32 = knw[0];
    let n: u32 = knw[1];
    let w: u32 = knw[2];

    let amt: u32 = k*(w)*(w+1)/2;
    let res: u32 = if amt > n { amt - n } else { 0 } ;
    println!("{}",res);
}