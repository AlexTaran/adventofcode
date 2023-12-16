use std::io;

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

#[derive(Clone, Copy, Eq, PartialEq)]
enum DIR {
    Up = 0,
    Left,
    Down,
    Right,
}


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
    let mut beams:Vec<Vec<[bool; 4]>> = vec![vec![[false, false, false, false];w];h];
    let isvalid = |v:&V2|v.x >= 0 && v.y >= 0 && v.x < w as i32 && v.y < h as i32;
    let mut front:Vec<(V2, DIR)> = Vec::new();
    front.push((V2{x: 0, y: 0}, DIR::Right));
    while front.len() > 0 {
        let mut newfront:Vec<(V2, DIR)> = Vec::new();
        for (pos, dir) in front.iter() {
            if beams[pos.y as usize][pos.x as usize][*dir as usize] {
                continue;
            }
            beams[pos.y as usize][pos.x as usize][*dir as usize] = true;
            let ch = table[pos.y as usize][pos.x as usize];
            let dlt = &DLT[*dir as usize];
            if ch == '.' {
                let npos = add(pos, dlt);
                if isvalid(&npos) {
                    newfront.push((npos, *dir));
                }
            } else if ch == '|' {
                if *dir == DIR::Up || *dir == DIR::Down {
                    let npos = add(pos, dlt);
                    if isvalid(&npos) {
                        newfront.push((npos, *dir));
                    }
                } else {
                    let n1pos = add(pos, &DLT[DIR::Up as usize]);
                    let n2pos = add(pos, &DLT[DIR::Down as usize]);
                    if isvalid(&n1pos) {
                        newfront.push((n1pos, DIR::Up));
                    }
                    if isvalid(&n2pos) {
                        newfront.push((n2pos, DIR::Down));
                    }
                }
            } else if ch == '-' {
                if *dir == DIR::Left || *dir == DIR::Right {
                    let npos = add(pos, dlt);
                    if isvalid(&npos) {
                        newfront.push((npos, *dir));
                    }
                } else {
                    let n1pos = add(pos, &DLT[DIR::Left as usize]);
                    let n2pos = add(pos, &DLT[DIR::Right as usize]);
                    if isvalid(&n1pos) {
                        newfront.push((n1pos, DIR::Left));
                    }
                    if isvalid(&n2pos) {
                        newfront.push((n2pos, DIR::Right));
                    }
                }
            } else if ch == '/' {
                let ndir = match *dir { 
                    DIR::Right => DIR::Up,
                    DIR::Left  => DIR::Down,
                    DIR::Down  => DIR::Left,
                    DIR::Up    => DIR::Right,
                };
                let npos = add(pos, &DLT[ndir as usize]);
                if isvalid(&npos) {
                    newfront.push((npos, ndir));
                }
            } else if ch == '\\' {
                let ndir = match *dir { 
                    DIR::Right => DIR::Down,
                    DIR::Left  => DIR::Up,
                    DIR::Down  => DIR::Right,
                    DIR::Up    => DIR::Left,
                };
                let npos = add(pos, &DLT[ndir as usize]);
                if isvalid(&npos) {
                    newfront.push((npos, ndir));
                }
            }
        }
        
        front = newfront;
    }
    let mut res = 0;
    for y in 0..h {
        for x in 0..w {
            let mut f = false;
            for d in 0..4 {
                if beams[y][x][d] {
                    f = true;
                }
            }
            if f {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
