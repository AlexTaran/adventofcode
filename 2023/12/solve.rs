use std::io;

fn solve(s: &Vec<char>, pos: usize, rest:&Vec<u32>) -> u32 {
    let mut cur = pos;
    while cur < s.len() && s[cur] == '.' {
        cur += 1;
    }
    if cur == s.len() {
        if rest.len() == 0 {
            return 1;
        } else {
            return 0;
        }
    }
    let mut ns = s.clone();
    let mut nrest = rest.clone();
    let mut res = 0;
    if s[cur] == '?' {
        ns[cur] = '.';
        res += solve(&ns, cur, &nrest);
        ns[cur] = '#';
    }
    if nrest.len() == 0 {
        return res;
    }
    let npos = cur+nrest[0] as usize;
    for i in cur..npos {
        if i >= ns.len() || ns[i] == '.' {
            return res;
        }
        ns[i] = '#';
    }
    if npos == ns.len() {
        if nrest.len() == 1 {
            res += 1;
        }
        return res;
    }
    if ns[npos] == '#' {
        return res;
    }
    ns[npos] = '.';
    nrest.remove(0);
    res += solve(&ns, npos, &nrest);
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
        let nums:Vec<u32> = parts[1].split(',').map(|x|x.parse::<u32>().unwrap()).collect();
        let s:Vec<char> = parts[0].chars().collect();
        res += solve(&s, 0, &nums);
    }
    println!("{}", res);
}
