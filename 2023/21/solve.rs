use std::io;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
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

fn main() {
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
    let mut start = V2 {x: 0, y: 0};
    for y in 0..h {
        for x in 0..w {
            if table[y][x] == 'S' {
                start = V2{x:x as i32, y:y as i32};
            }
        }
    }
    let mut reachable:HashSet<V2> = HashSet::new();
    reachable.insert(start);
    for _n in 0..64 {
        let mut nr:HashSet<V2> = HashSet::new();
        for pos in reachable.iter() {
            for dlt in DLT {
                let neib = add(pos, &dlt);
                if !isvalid(&neib) {
                    continue;
                }
                if table[neib.y as usize][neib.x as usize] == '#' {
                    continue;
                }
                nr.insert(neib);
            }
        }
        reachable = nr;
    }
    println!("{}", reachable.len());
}
