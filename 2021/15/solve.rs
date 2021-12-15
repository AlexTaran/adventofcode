use std::io;

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
        visited.push(vec![false; arr.len()]);
        labels.push(vec![u32::MAX; arr.len()]);
        table.push(arr);
    }
    labels[0][0] = 0;

    let dx = vec![-1, 1, 0,  0];
    let dy = vec![ 0, 0, 1, -1];
    loop {
        let mut x:i32 = 0;
        let mut y:i32 = 0;
        let mut found = false;
        for i in 0..table.len() {
            for j in 0..table[i].len() {
                if visited[i][j] {
                    continue;
                }
                if !found {
                    x = i as i32;
                    y = j as i32;
                    found = true;
                    continue;
                }
                if labels[i][j] < labels[x as usize][y as usize] {
                    x = i as i32;
                    y = j as i32;
                }
            }
        }
        if !found {
            break;
        }
        let label = labels[x as usize][y as usize];
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
                labels[nx as usize][ny as usize] = label + table[nx as usize][ny as usize];
            }
        }
        visited[x as usize][y as usize] = true;
    }
    println!("{}", labels[labels.len() - 1][labels[0].len() - 1]);
}
