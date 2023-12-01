use std::io;

fn main() {
    let digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut sum:u32 = 0;
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let mut d1:u32 = 0;
        'outer: for i in 0..line.len() {
            let ch = line.chars().nth(i).unwrap() as u32;
            if ch >= '0' as u32 && ch <= '9' as u32{
                d1 = ch - '0' as u32;
                break;
            } else {
                for (j, digit) in digits.iter().enumerate() {
                    let subs = &line[i..(i+digit.len()).min(line.len())];
                    if subs == *digit {
                        d1 = j as u32 + 1;
                        break 'outer;
                    }
                }
            }
        }
        let mut d2:u32 = 0;
        'outer2: for i in (0..line.len()).rev() {
            let ch = line.chars().nth(i).unwrap() as u32;
            if ch >= '0' as u32 && ch <= '9' as u32{
                d2 = ch - '0' as u32;
                break;
            } else {
                for (j, digit) in digits.iter().enumerate() {
                    let subs = &line[i..(i+digit.len()).min(line.len())];
                    if subs == *digit {
                        d2 = j as u32 + 1;
                        break 'outer2;
                    }
                }
            }
        }
        sum += d1 * 10 + d2;
    }
    println!("{}", sum);
}
