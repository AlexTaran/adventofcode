use std::io;
use std::cmp;
use std::collections::HashMap;

fn main() {
    let mut start_line = String::new();
    io::stdin().read_line(&mut start_line).unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();

    let mut mp:HashMap<String, String> = HashMap::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().len() == 0 {
            break;
        }
        let spl:Vec<&str> = line.trim().split(" -> ").collect();
        mp.insert(spl[0].to_string(), spl[1].to_string());
    }
    start_line = start_line.trim().to_string();
    for _step in 0..10 {
        let mut s = String::new();
        for i in 0..(start_line.len() - 1) {
            let key = &start_line[i..(i+2)];
            s.push(start_line.chars().nth(i).unwrap());
            if mp.contains_key(key) {
                s.push_str(mp.get(key).unwrap());
            }
        }
        s.push(start_line.chars().last().unwrap());

        start_line = s;
    }
    let mut cnt:HashMap<char, u32> = HashMap::new();
    for ch in start_line.chars() {
        if cnt.contains_key(&ch) {
            *cnt.get_mut(&ch).unwrap() += 1;
        } else {
            cnt.insert(ch, 1);
        }
    }
    let mut mx:u32 = 0;
    let mut mn:u32 = start_line.len() as u32;
    for val in cnt.values() {
        mx = cmp::max(mx, *val);
        mn = cmp::min(mn, *val);
    }

    println!("{}", mx - mn);
}
