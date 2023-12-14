use std::io;

fn main() {
    let mut table:Vec<Vec<char>> = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        table.push(line.trim().chars().collect());
    }
    for i in 0..table[0].len() {
        for j in 0..table.len() {
            if table[j][i] != 'O' {
                continue;
            }
            let mut d = j;
            while d > 0 && table[d-1][i] == '.' {
                d -= 1;
            }
            table[j][i] = '.';
            table[d][i] = 'O';
        }
    }
    let mut res = 0;
    for i in 0..table[0].len() {
        for j in 0..table.len() {
            if table[j][i] == 'O' {
                res += table.len() - j;
            }
        }
    }
    println!("{}", res);
}
