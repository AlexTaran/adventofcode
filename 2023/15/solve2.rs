use std::io;

fn hsh(s: &str) -> u32 {
    let mut res:u32 = 0;
    for ch in s.chars() {
        res += (ch as u8) as u32;
        res *= 17;
        res %= 256;
    }
    return res;
}

fn main() {
    let mut s = String::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        s = s + line.trim();
    }
    let parts:Vec<&str> = s.split(',').collect();
    let mut table:Vec<Vec<(String, u32)>> = vec![Vec::new(); 256];
    'outer: for p in parts.iter() {
        let pts:Vec<&str> = p.split(if p.contains('-') {'-'} else {'='}).collect();
        let idx = hsh(pts[0]);
        let v = &mut table[idx as usize];
        for i in 0..v.len() {
            if v[i].0 == pts[0] {
                if p.contains('-') {
                    v.remove(i);
                } else {
                    v[i] = (pts[0].to_string(), pts[1].parse::<u32>().unwrap());
                }
                continue 'outer;
            }
        }
        if p.contains('=') {
            v.push((pts[0].to_string(), pts[1].parse::<u32>().unwrap()));
        }
    }
    let mut res = 0;
    for b in 0..256 {
        for i in 0..table[b].len() {
            res += (b+1)*(i+1)*table[b][i].1 as usize;
        }
    }
    println!("{}", res);
}
