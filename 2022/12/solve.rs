use std::io;
use std::collections::VecDeque;

struct Pt {
    x: usize,
    y: usize,
}

fn main() {
    let mut table:Vec<Vec<u32>> = Vec::new();
    let mut sx = 0;
    let mut sy = 0;
    let mut ex = 0;
    let mut ey = 0;
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        if line.trim().len() == 0 {
            break;
        }
        let l = line.trim();
        let spos = l.chars().position(|c| c == 'S');
        let epos = l.chars().position(|c| c == 'E');
        if spos != None {
            sx = spos.unwrap();
            sy = table.len();
        }
        if epos != None {
            ex = epos.unwrap();
            ey = table.len();
        }
        table.push(l.replace("S", "a").replace("E", "z").chars().map(|c|c as u32).collect());
    }
    let h = table.len();
    let w = table[0].len();
    let mut dists = vec![vec![-1;w];h];
    dists[sy][sx] = 0;
    let mut q = VecDeque::new();
    q.push_back(Pt{x: sx, y: sy});
    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        let dist = dists[cur.y][cur.x];
        let val = table[cur.y][cur.x];
        if cur.x > 0 && dists[cur.y][cur.x - 1] == -1 && table[cur.y][cur.x - 1] <= val + 1 {
            dists[cur.y][cur.x - 1] = dist + 1;
            q.push_back(Pt{x: cur.x - 1, y: cur.y});
        }
        if cur.x + 1 < w && dists[cur.y][cur.x + 1] == -1 && table[cur.y][cur.x + 1] <= val + 1 {
            dists[cur.y][cur.x + 1] = dist + 1;
            q.push_back(Pt{x: cur.x + 1, y: cur.y});
        }
        if cur.y > 0 && dists[cur.y - 1][cur.x] == -1 && table[cur.y - 1][cur.x] <= val + 1 {
            dists[cur.y - 1][cur.x] = dist + 1;
            q.push_back(Pt{x: cur.x, y: cur.y - 1});
        }
        if cur.y + 1 < h && dists[cur.y + 1][cur.x] == -1 && table[cur.y + 1][cur.x] <= val + 1 {
            dists[cur.y + 1][cur.x] = dist + 1;
            q.push_back(Pt{x: cur.x, y: cur.y + 1});
        }
    }

    println!("{}", dists[ey][ex]);
}
