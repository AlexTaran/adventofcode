use std::io;

fn main() {
    let mut scores = Vec::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().len() == 0 {
            break;
        }
        let mut stk = Vec::new();
        let mut corr = true;
        for ch in line.trim().chars() {
            if ch == '(' || ch == '[' || ch == '{' || ch == '<' {
                stk.push(ch);
            } else {
                let ls = stk[stk.len() - 1];
                if (ls == '[' && ch == ']') || (ls == '<' && ch == '>') || (ls == '(' && ch == ')') || (ls == '{' && ch == '}') {
                    stk.pop();
                } else {
                    corr = false;
                    break;
                }
            }
        }
        if corr {
            let mut score:u64 = 0;
            while stk.len() > 0 {
                let ch = stk.pop().unwrap();
                score *= 5;
                if ch == '(' {
                    score += 1;
                } else if ch == '[' {
                    score += 2;
                } else if ch == '{' {
                    score += 3;
                } else if ch == '<' {
                    score += 4;
                }
            }
            scores.push(score);
        }
    }
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}
