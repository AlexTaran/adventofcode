use std::io;
use std::cmp::min;
use std::cmp::max;
use std::collections::HashSet;

fn diff(a: usize, b: usize) -> usize {
    if a > b {
        return a - b;
    }
    return b - a;
}

fn main() {
    let mut table:Vec<Vec<char>> = Vec::new();
    let mut rows = HashSet::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        if line.trim().chars().all(|c|c=='.') {
            rows.insert(table.len());
        }
        table.push(line.trim().chars().collect());
    }
    let mut cols = HashSet::new();
    for i in 0..table[0].len() {
        if (0..table.len()).all(|j|table[j][i] == '.') {
            cols.insert(i);
        }
    }
    let w = table[0].len();
    let h = table.len();
    let mut galaxies = Vec::new();
    for i in 0..w {
        for j in 0..h {
            if table[j][i] == '#' {
                galaxies.push((j, i));
            }
        }
    }
    let mut res = 0;
    for ga in 0..galaxies.len() {
        for gb in (ga+1)..galaxies.len() {
            let a = &galaxies[ga];
            let b = &galaxies[gb];
            res += diff(a.0, b.0) + diff(a.1, b.1);
            for j in min(a.0, b.0)..max(a.0, b.0) {
                if rows.contains(&j) {
                    res += 999999;
                }
            }
            for i in min(a.1, b.1)..max(a.1,b.1) {
                if cols.contains(&i) {
                    res += 999999;
                }
            }
        }
    }
    println!("{}", res);
}
