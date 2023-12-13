use std::io;

fn calc(table: &Vec<Vec<char>>) -> u32 {
    let mut res = 0;
    for r in 0..table.len()-1 {
        let w = std::cmp::min(r + 1, table.len() - r - 1);
        let mut ok = true;
        'outer: for j in 0..w {
            for i in 0..table[0].len() {
                if table[r-j][i] != table[r+1+j][i] {
                    ok = false;
                    break 'outer;
                }
            }
        }
        if ok {
            res += 100 * (r as u32 +1);
        }
    }
    for c in 0..table[0].len()-1 {
        let w = std::cmp::min(c + 1, table[0].len() - c - 1);
        let mut ok = true;
        'outer: for i in 0..w {
            for j in 0..table.len() {
                if table[j][c-i] != table[j][c+1+i] {
                    ok = false;
                    break 'outer;
                }
            }
        }
        if ok {
            res += c as u32 +1;
        }
    }
    return res;
}

fn main() {
    let mut res = 0;
    let mut table:Vec<Vec<char>> = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            res += calc(&table);
            break;
        }
        if line.trim().len() == 0 {
            res += calc(&table);
            table = Vec::new();
            continue;
        }
        table.push(line.trim().chars().collect());
    }
    println!("{}", res);
}
