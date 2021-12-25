use std::io;
use std::collections::HashSet;

fn step(t: &Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
    let mut changed = false;
    let mut res = t.clone();
    let mut cands = HashSet::new();
    for i in 0..t.len() {
        for j in 0.. t[i].len() {
            if t[i][j] != '>' {
                continue;
            }
            let r = (j + 1) % t[i].len();
            if t[i][r] == '.' {
                cands.insert((i, j));
            }
        }
    }
    for c in cands.iter() {
        res[c.0][c.1] = '.';
        let r = (c.1 + 1) % t[0].len();
        res[c.0][r] = '>';
    }
    if cands.len() > 0 {
        changed = true;
    }
    cands = HashSet::new();
    for i in 0..t.len() {
        for j in 0.. t[i].len() {
            if res[i][j] != 'v' {
                continue;
            }
            let b = (i + 1) % t.len();
            if res[b][j] == '.' {
                cands.insert((i, j));
            }
        }
    }
    for c in cands.iter() {
        res[c.0][c.1] = '.';
        let b = (c.0 + 1) % t.len();
        res[b][c.1] = 'v';
    }
    if cands.len() > 0 {
        changed = true;
    }
    return (res, changed);
}

fn main() {
    let mut table:Vec<Vec<char>> = Vec::new();

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().len() == 0 {
            break;
        }
        table.push(line.trim().chars().collect());
    }
    let mut cnt = 0;
    loop {
        let (nt, changed) = step(&table);
        cnt += 1;
        if !changed {
            break;
        }
        table = nt;
    }
    println!("{}", cnt);
}
