use std::io;
use std::cmp;
use std::collections::HashMap;

fn add(mp: &mut HashMap<String, u64>, key: &String, val: u64) {
    if mp.contains_key(key) {
        *mp.get_mut(key).unwrap() += val;
    } else {
        mp.insert(key.to_string(), val);
    }
}

fn main() {
    let mut start_line = String::new();
    io::stdin().read_line(&mut start_line).unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
    start_line = start_line.trim().to_string();

    let mut cur_mp:HashMap<String, u64> = HashMap::new();
    for i in 0..(start_line.len() - 1) {
        let key = start_line[i..(i+2)].to_string();
        if cur_mp.contains_key(&key) {
            *cur_mp.get_mut(&key).unwrap() += 1;
        } else {
            cur_mp.insert(key, 1);
        }
    }

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
    for _step in 0..40 {
        let mut nxt_mp:HashMap<String, u64> = HashMap::new();
        for (key, val) in cur_mp.iter() {
            if !mp.contains_key(key) {
                add(&mut nxt_mp, &key, *val);
                continue;
            }
            let ch = mp.get(key).unwrap().chars().nth(0).unwrap();
            let mut key1 = String::new();
            let mut key2 = String::new();
            key1.push(key.chars().nth(0).unwrap());
            key1.push(ch);
            key2.push(ch);
            key2.push(key.chars().nth(1).unwrap());
            add(&mut nxt_mp, &key1, *val);
            add(&mut nxt_mp, &key2, *val);
        }
        cur_mp = nxt_mp;
    }
    let mut cnt:HashMap<char, u64> = HashMap::new();
    cnt.insert('N', 1);
    cnt.insert('B', 1);
    for (key, val) in cur_mp.iter() {
        for ch in key.chars() {
            if cnt.contains_key(&ch) {
                *cnt.get_mut(&ch).unwrap() += *val;
            } else {
                cnt.insert(ch, *val);
            }
        }
    }
    let mut mx:u64 = 0;
    let mut mn:u64 = u64::MAX;
    for val in cnt.values() {
        mx = cmp::max(mx, *val);
        mn = cmp::min(mn, *val);
    }

    println!("{}", (mx - mn) / 2);
}
