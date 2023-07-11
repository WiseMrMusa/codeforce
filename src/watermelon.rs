use std::io;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");
    let input: u8 = match input.trim().parse(){
        Ok(value) => value,
        Err(_) => {
            panic!("Invalid Input");
        }
    };
    water(input);
}

fn water(x: u8){
    if x < 4 {
        println!("NO");
    } else {
        if x%2 == 0 {
            println!("YES");
        }
        else {
            println!("NO");
        }
    }
}