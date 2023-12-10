use std::io;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Hash, Eq, PartialEq, Clone)]
struct V2 {
    x: i32,
    y: i32,
}

fn add(a: &V2, b: &V2) -> V2 {
  return V2 {x: a.x+b.x, y: a.y+b.y};
}

const DLT: [V2; 4] = [
    V2 {x: 0, y: -1},
    V2 {x:-1, y:  0},
    V2 {x: 0, y:  1},
    V2 {x: 1, y:  0},
];

const RDLT: [V2; 4] = [
    V2{x: 0, y:-1},
    V2{x:-1, y:-1},
    V2{x:-1, y: 0},
    V2{x: 0, y: 0},
];

const LDLT: [V2; 4] = [
    V2{x:-1, y:-1},
    V2{x:-1, y: 0},
    V2{x: 0, y: 0},
    V2{x: 0, y:-1},
];

fn main() {
    let dirs:HashMap<char, (V2, V2)> = vec![
        ('J', (V2{x:-1, y: 0}, V2{x: 0, y:-1})),
        ('F', (V2{x: 1, y: 0}, V2{x: 0, y: 1})),
        ('L', (V2{x: 1, y: 0}, V2{x: 0, y:-1})),
        ('7', (V2{x:-1, y: 0}, V2{x: 0, y: 1})),
        ('-', (V2{x:-1, y: 0}, V2{x: 1, y: 0})),
        ('|', (V2{x: 0, y:-1}, V2{x: 0, y: 1})),
    ].into_iter().collect();
    let mut table:Vec<Vec<char>> = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        table.push(line.trim().chars().collect());
    }
    let w = table[0].len();
    let h = table.len();
    let isvalid = |v:&V2|v.x >= 0 && v.y >= 0 && v.x < w as i32 && v.y < h as i32;
    let mut start:V2 = V2 {x: 0, y: 0};
    for i in 0..h {
        for j in 0..w {
            if table[i][j] == 'S' {
                start = V2 {x: j as i32, y: i as i32};
            }
        }
    }
    let mut dists: Vec<Vec<i32>> = vec![vec![-1; w]; h];

    dists[start.y as usize][start.x as usize] = 0;
    let mut loopmax = 0;
    'outer: for dlt in DLT {
        let mut curr:V2 = start.clone();
        let neib = add(&curr, &dlt);
        if !isvalid(&neib) {
            continue;
        }
        if !dirs.contains_key(&table[neib.y as usize][neib.x as usize]) {
            continue;
        }
        let pair = dirs.get(&table[neib.y as usize][neib.x as usize]).unwrap();
        if add(&neib, &pair.0) != start && add(&neib, &pair.1) != start {
            continue;
        }
        curr = neib.clone();
        loop {
            let p = dirs.get(&table[curr.y as usize][curr.x as usize]).unwrap();
            let n0 = add(&curr, &p.0);
            let n1 = add(&curr, &p.1);
            let d0 = dists[n0.y as usize][n0.x as usize];
            let d1 = dists[n1.y as usize][n1.x as usize];
            if  d0 != -1 && d1 != -1 {
                loopmax = d0+d1+1;
                dists[curr.y as usize][curr.x as usize] = loopmax;
                break 'outer;
            }
            if d0 != -1 {
                dists[curr.y as usize][curr.x as usize] = d0 + 1;
                curr = n1;
                continue;
            }
            if d1 != -1 {
                dists[curr.y as usize][curr.x as usize] = d1 + 1;
                curr = n0;
                continue;
            }
            println!("Err");
            break;
        }
    }
    for i in 0..h {
        for j in 0..w {
            if dists[i][j] == -1 {
                table[i][j] = '.';
            }
        }
    }
    let mut visited: Vec<Vec<bool>> = vec![vec![false; w+1]; h+1];
    let mut q = VecDeque::new();
    q.push_back(V2{x: 0, y: 0});
    visited[0][0] = true;
    while q.len() > 0 {
        let curr = q.pop_front().unwrap();
        for i in 0..4 {
            let neib = add(&curr, &DLT[i]);
            if neib.x < 0 || neib.y < 0 || neib.x > w as i32 || neib.y > h as i32{
                continue;
            }
            if visited[neib.y as usize][neib.x as usize] {
                continue;
            }
            let cl = add(&curr, &LDLT[i]);
            let cr = add(&curr, &RDLT[i]);
            if isvalid(&cl) && isvalid(&cr) {
                let dl = dists[cl.y as usize][cl.x as usize];
                let dr = dists[cr.y as usize][cr.x as usize];
                if dl + 1 == dr || dr + 1 == dl || (dl == 0 && dr == loopmax) || (dr == 0 && dl == loopmax) {
                    continue;
                }
            }
            visited[neib.y as usize][neib.x as usize] = true;
            q.push_back(neib);
        }
    }
    for i in 0..h {
        for j in 0..w {
            if visited[i][j] && table[i][j] == '.' {
                table[i][j] = 'O';
            }
        }
    }
    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if table[i][j] == '.' {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
