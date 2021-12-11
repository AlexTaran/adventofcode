use std::io;

fn main() {
    let mut table:Vec<Vec<u32>> = Vec::new();

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().len() == 0 {
            break;
        }
        let arr:Vec<u32> = line.trim().chars().map(|x|x.to_string().parse::<u32>().unwrap()).collect();
        table.push(arr);
    }

    let dx = vec![-1, -1,  1, 1, -1, 1, 0,  0];
    let dy = vec![-1,  1, -1, 1,  0, 0, 1, -1];

    let mut step = 0;
    loop {
        step += 1;
        let mut t = table.clone();
        for i in 0..10 {
            for j in 0..10 {
                t[i][j] += 1;
            }
        }
        let mut flags:Vec<Vec<bool>> = Vec::new();
        for _i in 0..10 {
            flags.push(vec![false; 10]);
        }
        loop {
            let mut flashes:Vec<(usize, usize)> = Vec::new();
            for i in 0..10 {
                for j in 0..10 {
                    if flags[i][j] {
                        continue;
                    }
                    if t[i][j] > 9 {
                        t[i][j] = 0;
                        flashes.push((i, j));
                        flags[i][j] = true;
                    }
                }
            }
            for (x, y) in flashes.iter() {
                for d in 0..8 {
                    let nx:i32 = *x as i32 + dx[d];
                    let ny:i32 = *y as i32 + dy[d];
                    if nx < 0 || ny < 0 || nx > 9 || ny > 9 {
                        continue;
                    }
                    if flags[nx as usize][ny as usize] {
                        continue;
                    }
                    t[nx as usize][ny as usize] += 1;
                }
            }
            if flashes.len() == 0 {
                break;
            }
        }
        table = t;
        let mut stop = true;
        for i in 0..10 {
            for j in 0..10 {
                stop &= flags[i][j];
            }
        }
        if stop {
            break;
        }
    }
    println!("{}", step);
}
