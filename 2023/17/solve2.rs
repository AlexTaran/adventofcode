use std::io;
use std::collections::BTreeSet;

#[derive(Hash, Eq, PartialEq, Clone)]
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
    let mut table:Vec<Vec<u32>> = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        table.push(line.trim().chars().map(|ch|ch.to_digit(10).unwrap()).collect());
    }
    let w = table[0].len();
    let h = table.len();
    let mut visited:Vec<Vec<[[bool; 4]; 10]>> = vec![vec![[[false; 4]; 10];w];h];
    let mut labels:Vec<Vec<[[u32; 4]; 10]>> = vec![vec![[[u32::MAX; 4]; 10];w];h];
    let isvalid = |v:&V2|v.x >= 0 && v.y >= 0 && v.x < w as i32 && v.y < h as i32;

    // Initialization.
    labels[0][1][0][3 /* Right */] = table[0][1];
    labels[1][0][0][2 /* Down */] = table[1][0];

    let mut mp:BTreeSet<(u32, usize, usize, usize, usize)> = BTreeSet::new();
    for y in 0..h {
        for x in 0..w {
            for i in 0..10 {
                for j in 0..4 {
                    mp.insert( (labels[y][x][i][j], y, x, i, j) );
                }
            }
        }
    }

    loop {
        // Check if answer is ready.
        let mut ready = true;
        for i in 0..10 {
            for j in 0..4 {
                if !visited[h-1][w-1][i][j] {
                    ready = false;
                }
            }
        }
        if ready {
            break;
        }
        // Finding unvisited vertex with minimal label.
        let elem = mp.iter().next().unwrap().clone();
        let curlabel = elem.0;
        let coord = (elem.1, elem.2, elem.3, elem.4);
        // Relaxation edges from coord.
        if curlabel == u32::MAX {
            break;
        }
        let pos = V2 {x: coord.1 as i32, y: coord.0 as i32};
        let dir_turnl = (coord.3 + 1) % 4;
        let dir_turnr = (coord.3 + 3) % 4;
        let pos_turnl = add(&pos, &DLT[dir_turnl]);
        let pos_turnr = add(&pos, &DLT[dir_turnr]);
        
        if coord.2 >= 3 {
            if isvalid(&pos_turnl) && !visited[pos_turnl.y as usize][pos_turnl.x as usize][0][dir_turnl] {
                let candlabel = curlabel + table[pos_turnl.y as usize][pos_turnl.x as usize];
                let neiblabel = labels[pos_turnl.y as usize][pos_turnl.x as usize][0][dir_turnl];
                if candlabel < neiblabel {
                    mp.remove(&(neiblabel, pos_turnl.y as usize, pos_turnl.x as usize, 0, dir_turnl));
                    labels[pos_turnl.y as usize][pos_turnl.x as usize][0][dir_turnl] = candlabel;
                    mp.insert((candlabel, pos_turnl.y as usize, pos_turnl.x as usize, 0, dir_turnl));
                }
            }
            if isvalid(&pos_turnr) && !visited[pos_turnr.y as usize][pos_turnr.x as usize][0][dir_turnr] {
                let candlabel = curlabel + table[pos_turnr.y as usize][pos_turnr.x as usize];
                let neiblabel = labels[pos_turnr.y as usize][pos_turnr.x as usize][0][dir_turnr];
                if candlabel < neiblabel {
                    mp.remove(&(neiblabel, pos_turnr.y as usize, pos_turnr.x as usize, 0, dir_turnr));
                    labels[pos_turnr.y as usize][pos_turnr.x as usize][0][dir_turnr] = candlabel;
                    mp.insert((candlabel, pos_turnr.y as usize, pos_turnr.x as usize, 0, dir_turnr));
                }
            }
        }
        if coord.2 < 9 { // Can go forward.
            let pos_fwd = add(&pos, &DLT[coord.3]);
            if isvalid(&pos_fwd) && !visited[pos_fwd.y as usize][pos_fwd.x as usize][coord.2 + 1][coord.3] {
                let candlabel = curlabel + table[pos_fwd.y as usize][pos_fwd.x as usize];
                let neiblabel = labels[pos_fwd.y as usize][pos_fwd.x as usize][coord.2 + 1][coord.3];
                if candlabel < neiblabel {
                    mp.remove(&(neiblabel, pos_fwd.y as usize, pos_fwd.x as usize, coord.2 + 1, coord.3));
                    labels[pos_fwd.y as usize][pos_fwd.x as usize][coord.2 + 1][coord.3] = candlabel;
                    mp.insert((candlabel, pos_fwd.y as usize, pos_fwd.x as usize, coord.2 + 1, coord.3));
                }
            }
        }
        visited[coord.0][coord.1][coord.2][coord.3] = true;
        mp.remove(&elem);
    }

    let mut ans = u32::MAX;
    for i in 3..10 {
        for j in 0..4 {
            if labels[h-1][w-1][i][j] < ans {
                ans = labels[h-1][w-1][i][j];
            }
        }
    }
    println!("{}", ans);
}
