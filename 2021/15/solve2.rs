use std::io;
use std::collections::BTreeSet;

fn incw(val: u32, dlt: u32) -> u32 {
    let mut res = val + dlt;
    while res > 9 {
        res -= 9;
    }
    return res;
}

fn main() {
    let mut table:Vec<Vec<u32>> = Vec::new();
    let mut visited:Vec<Vec<bool>> = Vec::new();
    let mut labels:Vec<Vec<u32>> = Vec::new();

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().len() == 0 {
            break;
        }
        let arr:Vec<u32> = line.trim().chars().map(|x|x.to_string().parse::<u32>().unwrap()).collect();
        table.push(arr);
    }
    for i in 0..table.len() {
        let z = table[i].len();
        for s in 1..5 {
            for j in 0..z {
                let val = incw(table[i][j], s);
                table[i].push(val);
            }
        }
    }
    let z = table.len();
    for s in 1..5 {
        for i in 0..z {
            let mut v = Vec::new();
            for j in 0..table[0].len() {
                let val = incw(table[i][j], s);
                v.push(val);
            }
            table.push(v);
        }
    }
    for i in 0..table.len() {
        visited.push(vec![false; table[i].len()]);
        labels.push(vec![u32::MAX; table[i].len()]);
    }
    labels[0][0] = 0;

    let mut mp:BTreeSet<(u32, usize, usize)> = BTreeSet::new();
    for i in 0..labels.len() {
        for j in 0..labels[i].len() {
            mp.insert( (labels[i][j], i, j));
        }
    }

    let dx = vec![-1, 1, 0,  0];
    let dy = vec![ 0, 0, 1, -1];
    loop {
        if mp.len() == 0 {
            break;
        }
        let elem = mp.iter().next().unwrap().clone();
        let label = elem.0;
        let x:i32 = elem.1 as i32;
        let y:i32 = elem.2 as i32;
        for k in 0..4 {
            let nx = x + dx[k];
            let ny = y + dy[k];
            if nx < 0 || ny < 0 || nx >= table.len() as i32 || ny >= table[0].len() as i32 {
                continue;
            }
            if visited[nx as usize][ny as usize] {
                continue;
            }
            if label + table[nx as usize][ny as usize] < labels[nx as usize][ny as usize] {
                let key = (labels[nx as usize][ny as usize], nx as usize, ny as usize);
                mp.remove(&key);
                labels[nx as usize][ny as usize] = label + table[nx as usize][ny as usize];
                mp.insert( (labels[nx as usize][ny as usize], nx as usize, ny as usize) );
            }
        }
        visited[x as usize][y as usize] = true;
        mp.remove(&elem);
    }
    println!("{}", labels[labels.len() - 1][labels[0].len() - 1]);
}
