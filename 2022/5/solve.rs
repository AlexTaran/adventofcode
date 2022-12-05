use std::io;

fn main() {
    let mut crates:Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        if line.trim().len() == 0 {
            break;
        }
        crates.push(line.to_string());
    }
    let mut table:Vec<Vec<char>> = Vec::new();
    for idx in 0..crates.last().unwrap().len() {
        let ch = crates.last().unwrap().chars().nth(idx).unwrap();
        if ch == ' ' {
            continue;
        }
        let mut col:Vec<char> = Vec::new();
        for j in (0..crates.len()-1).rev() {
            let c = crates[j].chars().nth(idx);
            if c != None && c.unwrap() != ' ' {
                col.push(c.unwrap());
            }
        }
        table.push(col);
    }
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split(" ").collect();
        let x = parts[1].parse::<u32>().unwrap();
        let f = parts[3].parse::<usize>().unwrap()-1;
        let t = parts[5].parse::<usize>().unwrap()-1;
        for _step in 0..x {
            let ch:char = *table[f].last().unwrap();
            table[t].push(ch);
            table[f].pop();
        }
    }
    let mut lasts = Vec::new();
    for col in table {
        let ch:char = *col.last().unwrap();
        lasts.push(ch.to_string());
    }
    println!("{}", lasts.join("").trim());
}
