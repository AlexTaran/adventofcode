use std::io;

fn main() {
    let mut sm:u32 = 0;
    let mut lines:Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        if line.trim().len() == 0 {
            break;
        }
        lines.push(line.trim().to_string());
    }
    let mut cur = 0;
    loop {
        if cur >= lines.len() {
            break;
        }
        for i in 0..lines[cur].len() {
            let ch = lines[cur].chars().nth(i).unwrap();
            let mut ok1 = false;
            let mut ok2 = false;
            for j in 0..lines[cur+1].len() {
                if lines[cur+1].chars().nth(j).unwrap() == ch {
                    ok1 = true;
                    break;
                }
            }
            for j in 0..lines[cur+2].len() {
                if lines[cur+2].chars().nth(j).unwrap() == ch {
                    ok2 = true;
                    break;
                }
            }
            if ok1 && ok2 {
                let code = ch as u32;
                if code >= 'a' as u32 && code <= 'z' as u32 {
                  sm += code - 'a' as u32 + 1;
                } else {
                  sm += code - 'A' as u32 + 27;
                }
                break;
            }
        }
        cur += 3;
    }
    println!("{}", sm);
}
