use std::io::{ self, BufRead };

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap());

    let t: u16 = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let s = lines.next().unwrap().trim().to_owned();
        let m: u64 = lines.next().unwrap().trim().parse().unwrap();
        let l: u64 = lines.next().unwrap().trim().parse().unwrap();
        let n: u64 = lines.next().unwrap().trim().parse().unwrap();

        let s_chars: Vec<char> = s.chars().collect();
        let l_chars: Vec<char> = l.to_string().chars().collect();
        let n_chars: Vec<char> = n.to_string().chars().collect();

        let mut res = false;
        for i in l..=n {
            let i_chars: Vec<char> = i.to_string().chars().collect();
            res = check_substring(&s_chars, m, &i_chars, &l_chars, &n_chars);
            if res {
                break;
            }
        }
        println!("{}", if res { "YES" } else { "NO" });
    }
}

fn check_substring(s: &[char], m: u64, i: &[char], l: &[char], n: &[char]) -> bool {
    if s.is_empty() || i.is_empty() {
        true
    } else if s.len() as u64 > m && s[1] == s[0] {
        check_substring(&s[1..], m, &i, &l, &n)
    } else if i[0] < l[0] || i[0] > n[0] {
        false
    } else if s[0] != i[0] {
        check_substring(&s[1..], m, &i, &l, &n)
    } else {
        if m == 1 {
            false
        } else {
            check_substring(&s[1..], m - 1, &i[1..], &l[1..], &n[1..])
        }
    }
}