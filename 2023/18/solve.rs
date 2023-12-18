use std::io;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct V2 {
    x: i32,
    y: i32,
}

fn add(a: &V2, b: &V2) -> V2 {
    return V2 {x: a.x+b.x, y: a.y+b.y};
}

const DLT: [V2; 4] = [ 
    V2 {x: 0, y: -1}, // Up
    V2 {x:-1, y:  0}, // Left
    V2 {x: 0, y:  1}, // Down
    V2 {x: 1, y:  0}, // Right
];

fn main() {
    let ch2dir:HashMap<char, usize> = [('U', 0), ('L', 1), ('D', 2), ('R', 3)].iter().cloned().collect();
    let mut leftup = V2 {x: 0, y: 0};
    let mut rightdown = V2 {x: 0, y: 0};
    let mut cur = V2 {x: 0, y: 0};
    let mut route:Vec<(char, i32)> = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split(" ").collect();
        let dir = parts[0].chars().nth(0).unwrap();
        let dist = parts[1].parse::<i32>().unwrap();
        route.push((dir, dist));
        for _ in 0..dist {
            cur = add(&cur, &DLT[*ch2dir.get(&dir).unwrap()]);
        }
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
    leftup = add(&leftup, &V2{x: -1, y: -1});
    rightdown = add(&rightdown, &V2{x: 1, y: 1});
    let w = (rightdown.x - leftup.x + 1) as usize;
    let h = (rightdown.y - leftup.y + 1) as usize;
    let mut table:Vec<Vec<char>> = vec![vec!['.';w];h];
    cur = V2 {x: 0, y: 0};
    for (dir, dist) in route.iter() {
        for _ in 0..*dist {
            cur = add(&cur, &DLT[*ch2dir.get(&dir).unwrap()]);
            table[(cur.y - leftup.y) as usize][(cur.x - leftup.x) as usize] = '#';
        }
    }
    let mut visited:Vec<Vec<bool>> = vec![vec![false;w];h];
    let mut q = VecDeque::new();
    q.push_back(V2{x: 0, y: 0});
    visited[0][0] = true;
    while q.len() > 0 {
        let curr = q.pop_front().unwrap();
        for i in 0..4 {
            let neib = add(&curr, &DLT[i]);
            if neib.x < 0 || neib.y < 0 || neib.x >= w as i32 || neib.y >= h as i32 {
                continue;
            }
            if visited[neib.y as usize][neib.x as usize] || table[neib.y as usize][neib.x as usize] == '#'{
                continue;
            }
            visited[neib.y as usize][neib.x as usize] = true;
            q.push_back(neib);
        }
    }
    let mut cnt = 0;
    for x in 0..w {
        for y in 0..h {
            if !visited[y][x] {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
