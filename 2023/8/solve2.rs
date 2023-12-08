use std::io;
use std::collections::HashMap;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let mut instructions = String::new();
    io::stdin().read_line(&mut instructions).expect("");
    let chs:Vec<char> = instructions.trim().chars().collect();
    
    // skip empty str
    io::stdin().read_line(&mut instructions).expect("");

    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    let mut pos:Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let parts:Vec<&str> = line.split_terminator([' ', '=', '(', ',', ')']).filter(|&s| !s.is_empty()).collect();
        graph.insert(parts[0].to_string(), (parts[1].to_string(), parts[2].to_string()));
        if parts[0].chars().nth(2).unwrap() == 'A' {
            pos.push(parts[0].to_string());
        }
    }

    let mut res: u64 = 0;

    for posi in pos {
        let mut counter:u32 = 0;
        let mut curr:usize = 0;
        let mut p = posi;
        loop {
            if p.ends_with('Z') {
                break;
            }
            let ch = chs[curr];
            let entry = graph.get(&p).unwrap();
            if ch == 'L' {
                p = entry.0.to_string();
            } else {
                p = entry.1.to_string();
            }
 
            counter += 1;
            curr += 1;
            if curr == chs.len() {
                curr = 0;
            }
        }
        if res == 0 {
            res = counter as u64;
        } else {
            res = res * counter as u64 / gcd(res, counter as u64);
        }
    }
    println!("{}", res);
}
