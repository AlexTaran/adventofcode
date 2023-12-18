use std::io;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct V2 {
    x: i32,
    y: i32,
}

fn add(a: &V2, b: &V2) -> V2 {
    return V2 {x: a.x+b.x, y: a.y+b.y};
}

fn mul(v: &V2, n: i32) -> V2 {
    return V2 {x: v.x*n, y: v.y*n};
}

const DLT: [V2; 4] = [ 
    V2 {x: 1, y:  0}, // Right
    V2 {x: 0, y:  1}, // Down
    V2 {x:-1, y:  0}, // Left
    V2 {x: 0, y: -1}, // Up
];

fn main() {
    let mut leftup = V2 {x: 0, y: 0};
    let mut rightdown = V2 {x: 0, y: 0};
    let mut cur = V2 {x: 0, y: 0};
    let mut route:Vec<(usize, i32)> = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split(" ").collect();
        let dir = parts[2].chars().nth(7).unwrap().to_digit(10).unwrap() as usize;
        let dist = u64::from_str_radix(&parts[2][2..7], 16).unwrap() as i32;
        route.push((dir, dist));
        cur = add(&cur, &mul(&DLT[dir], dist));
        if cur.x > rightdown.x {
            rightdown.x = cur.x;
        }
        if cur.y > rightdown.y {
            rightdown.y = cur.y;
        }
        if cur.x < leftup.x {
            leftup.x = cur.x;
        }
        if cur.y < leftup.y {
            leftup.y = cur.y;
        }
    }
    let w = (rightdown.x - leftup.x + 1) as usize;
    // (start, finish, samedir)
    let mut borders:Vec<Vec<(i32, i32, bool)>> = vec![Vec::new(); w];
    cur = V2 {x: 0, y: 0};
    for idx in 0..route.len() {
        let dir = &route[idx].0;
        let dist = &route[idx].1;
        let ncur = add(&cur, &mul(&DLT[*dir], *dist));
        if dir % 2 == 0 { // Horizontal
            let mn = std::cmp::min(cur.x, ncur.x);
            let mx = std::cmp::max(cur.x, ncur.x);
            let y = cur.y - leftup.y;
            for x in mn+1..mx {
                borders[(x - leftup.x) as usize].push((y, y, false));
            }
        } else { // Vertical
            let mn = std::cmp::min(cur.y - leftup.y, ncur.y - leftup.y);
            let mx = std::cmp::max(cur.y - leftup.y, ncur.y - leftup.y);
            let nextdir = &route[(idx+1)%route.len()].0;
            let prevdir = &route[(idx+route.len()-1)%route.len()].0;
            let samedir = nextdir != prevdir;
            borders[(cur.x - leftup.x) as usize].push((mn, mx, samedir));
        }
        cur = ncur;
    }
    let mut res:u64 = 0;
    for brd in 0..borders.len() {
        let mut s = borders[brd].clone();
        s.sort();
        let mut inside = false;
        for i in 0..s.len() {
            if inside {
                res += (s[i].1 - s[i-1].1) as u64;
                if !s[i].2 {
                    inside = false;
                }
            } else {
                res += (s[i].1 - s[i].0) as u64 + 1;
                if !s[i].2 {
                    inside = true;
                }
            }
        }
    }
    println!("{}", res);
}
