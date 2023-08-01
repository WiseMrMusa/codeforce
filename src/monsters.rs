use std::{io, cmp::Ordering};

#[derive(Debug, PartialEq, Eq)]
struct Monster {
    hp: u32,
    index: usize,
}

impl Ord for Monster {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hp == other.hp {
            self.index.cmp(&other.index)
        } else {
            other.hp.cmp(&self.hp)
        }
    }
}

impl PartialOrd for Monster {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("Error reading line");    
        let mut monsters = a.split_whitespace().enumerate().map(|(i, x)| {
        let mut val = (x.parse::<u32>().unwrap() as u32) % k as u32;
            if val == 0 {
                val += k as u32;
            }
            Monster{ hp: val,  index: i+1 }
        }).collect::<Vec<_>>();
        monsters.sort();
        for m in monsters {
            print!("{} ", m.index );
        }
        println!(" ");
    }
}
