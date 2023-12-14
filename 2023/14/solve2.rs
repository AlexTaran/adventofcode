use std::io;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash(table: &Vec<Vec<char>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    for i in 0..table[0].len() {
        for j in 0..table.len() {
            table[j][i].hash(&mut hasher);
        }
    }
    return hasher.finish();
}

fn cycle(inp: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut table = inp.clone();
    // north
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
    // west
    for j in 0..table.len() {
        for i in 0..table[0].len() {
            if table[j][i] != 'O' {
                continue;
            }
            let mut d = i;
            while d > 0 && table[j][d-1] == '.' {
                d -= 1;
            }
            table[j][i] = '.';
            table[j][d] = 'O';
        }
    }
    // south
    for i in 0..table[0].len() {
        for j in (0..table.len()).rev() {
            if table[j][i] != 'O' {
                continue;
            }
            let mut d = j;
            while d + 1 < table.len() && table[d+1][i] == '.' {
                d += 1;
            }
            table[j][i] = '.';
            table[d][i] = 'O';
        }
    }
    // east
    for j in 0..table.len() {
        for i in (0..table[0].len()).rev() {
            if table[j][i] != 'O' {
                continue;
            }
            let mut d = i;
            while d + 1 < table[0].len() && table[j][d+1] == '.' {
                d += 1;
            }
            table[j][i] = '.';
            table[j][d] = 'O';
        }
    }
    return table;
}

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
    let mut known:HashMap<u64, u32> = HashMap::new();
    let mut n = 0;
    let max = 1000000000;
    while n < max {
        let h = hash(&table);
        if known.contains_key(&h) {
            let diff = n - known.get(&h).unwrap();
            n += (max-n)/diff*diff;
            break;
        }
        known.insert(h, n);
        table = cycle(&table);
        n += 1;
    }
    while n < max {
        table = cycle(&table);
        n += 1;
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
