use std::io;

fn hsh(s: &str) -> u32 {
    let mut res:u32 = 0;
    for ch in s.chars() {
        res += (ch as u8) as u32;
        res *= 17;
        res %= 256;
    }
    return res;
}

fn main() {
    let mut res = 0;
    let mut s = String::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        s = s + line.trim();
    }
    let parts:Vec<&str> = s.split(',').collect();
    for p in parts.iter() {
        res += hsh(p);
    }
    println!("{}", res);
}
