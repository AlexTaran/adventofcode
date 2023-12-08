use std::io;
use std::collections::HashMap;

fn main() {
    let mut instructions = String::new();
    io::stdin().read_line(&mut instructions).expect("");
    let chs:Vec<char> = instructions.trim().chars().collect();
    
    // skip empty str
    io::stdin().read_line(&mut instructions).expect("");

    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let parts:Vec<&str> = line.split_terminator([' ', '=', '(', ',', ')']).filter(|&s| !s.is_empty()).collect();
        graph.insert(parts[0].to_string(), (parts[1].to_string(), parts[2].to_string()));
    }

    let mut counter:u32 = 0;
    let mut curr:usize = 0;
    let mut pos = "AAA";
    loop {
        if pos == "ZZZ" {
            break;
        }
        let ch = chs[curr];
        let entry = graph.get(pos).unwrap();
        if ch == 'L' {
            pos = &entry.0;
        } else {
            pos = &entry.1;
        }

        counter += 1;
        curr += 1;
        if curr == chs.len() {
            curr = 0;
        }
    }
    println!("{}", counter);
}
