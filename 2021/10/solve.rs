use std::io;

fn main() {

    let mut cnt:u64 = 0;
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().len() == 0 {
            break;
        }
        let mut stk = Vec::new();
        for ch in line.trim().chars() {
            if ch == '(' || ch == '[' || ch == '{' || ch == '<' {
                stk.push(ch);
            } else {
                let ls = stk[stk.len() - 1];
                if (ls == '[' && ch == ']') || (ls == '<' && ch == '>') || (ls == '(' && ch == ')') || (ls == '{' && ch == '}') {
                    stk.pop();
                } else {
                    if ch == ')' {
                        cnt += 3;
                    } else if ch == ']' {
                        cnt += 57;
                    } else if ch == '}' {
                        cnt += 1197;
                    } else if ch == '>' {
                        cnt += 25137;
                    }
                    break;
                }
            }
        }
    }
    println!("{}", cnt);
}
