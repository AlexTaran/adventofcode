use std::io;

fn main() {
    let mut sum:u32 = 0;
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let mut d1:u32 = 0;
        for i in 0..line.len() {
            let ch = line.chars().nth(i).unwrap() as u32;
            if ch >= '0' as u32 && ch <= '9' as u32{
                d1 = ch - '0' as u32;
                break;
            }
        }
        let mut d2:u32 = 0;
        for i in (0..line.len()).rev() {
            let ch = line.chars().nth(i).unwrap() as u32;
            if ch >= '0' as u32 && ch <= '9' as u32{
                d2 = ch - '0' as u32;
                break;
            }
        }
        sum += d1 * 10 + d2;
    }
    println!("{}", sum);
}
