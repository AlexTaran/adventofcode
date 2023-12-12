use std::io;
use std::collections::HashMap;

fn solve(cache: &mut HashMap<(usize, usize), u64>, s: &mut Vec<char>, pos: usize, rest:&Vec<u32>, rpos: usize) -> u64 {
    let k = (pos, rpos);
    if cache.contains_key(&k) {
        return *cache.get(&k).unwrap();
    }
    let mut cur = pos;
    while cur < s.len() && s[cur] == '.' {
        cur += 1;
    }
    if cur == s.len() {
        if rpos == rest.len() {
            cache.insert(k, 1);
            return 1;
        } else {
            cache.insert(k, 0);
            return 0;
        }
    }
    if rpos < rest.len() {
        if rest[rpos..].iter().sum::<u32>() as usize + rest.len() - rpos - 1 > s.len() - pos {
            cache.insert(k, 0);
            return 0;
        }
    }
    let mut res = 0;
    if s[cur] == '?' {
        res += solve(cache, s, cur+1, &rest, rpos);
    }
    if rpos == rest.len() {
        cache.insert(k, res);
        return res;
    }
    let npos = cur+rest[rpos] as usize;
    for i in cur..npos {
        if i >= s.len() || s[i] == '.' {
            cache.insert(k, res);
            return res;
        }
    }
    if npos == s.len() {
        if rpos + 1 == rest.len() {
            res += 1;
        }
        cache.insert(k, res);
        return res;
    }
    if s[npos] == '#' {
        cache.insert(k, res);
        return res;
    }
    res += solve(cache, s, npos+1, &rest, rpos+1);
    cache.insert(k, res);
    return res;
}

fn main() {
    let mut res = 0;
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split_whitespace().collect();
        let mut nums:Vec<u32> = parts[1].split(',').map(|x|x.parse::<u32>().unwrap()).collect();
        let mut s:Vec<char> = parts[0].chars().collect();
        nums = (0..5).flat_map(|_|nums.clone()).collect();
        s = (0..5).flat_map(|i|{let mut ss = s.clone(); if i != 4 {ss.push('?');}; ss}).collect();
        let mut cache:HashMap<(usize, usize), u64> = HashMap::new();
        let v = solve(&mut cache, &mut s, 0, &nums, 0);
        res += v;
    }
    println!("{}", res);
}
