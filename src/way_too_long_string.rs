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

    for _hello in 0..input{
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Error reading line");
        shorten(&mut word);
    }
    
}

fn shorten(word: &mut String) {
    let word = word.trim();
    if word.len() < 11 {
        println!("{}", word)
    } else {
        let removed_char = word.len() - 2;
        let removed_char: Vec<char> = removed_char.to_string().chars().rev().take(3).collect();
        
        let mut result = word.to_string().chars().next().unwrap().to_string();
        let last_char: Vec<char> = word.to_string().chars().rev().take(3).collect();
        if word.len() >= 12 {
            result.push(removed_char[1]);
        }
        result.push(removed_char[0]);
        result.push(last_char[0]);
        println!("{}", result);
    }
}