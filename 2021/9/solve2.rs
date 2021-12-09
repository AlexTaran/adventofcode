use std::io;
use std::collections::VecDeque;

fn main() {
    let mut arr:Vec<Vec<u32>> = Vec::new();
    let mut vis:Vec<Vec<u32>> = Vec::new();

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().len() == 0 {
            break;
        }
        let v:Vec<u32> = line.trim().chars().map(|x|x.to_string().parse::<u32>().unwrap()).collect();
        let mut vr = vec![0; v.len()];
        for i in 0..v.len() {
            if v[i] == 9 {
                vr[i] = 1;
            }
        }
        arr.push(v);
        vis.push(vr);
    }
    let mut q = VecDeque::new();
    let mut szs:Vec<u32> = Vec::new();
    let dx:Vec<i32> = vec!(-1,  0, 1, 0);
    let dy:Vec<i32> = vec!( 0, -1, 0, 1);
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if vis[i][j] == 1 {
                continue;
            }
            let mut cnt = 0;
            q.push_back( (i, j) );
            vis[i][j] = 1;
            while q.len() > 0 {
                let (x, y) = q.pop_front().unwrap();
                cnt += 1;
                for d in 0..4 {
                    let nx = x as i32 + dx[d];
                    let ny = y as i32 + dy[d];
                    if nx < 0 || ny < 0 || nx >= arr.len() as i32 || ny >= arr[i].len() as i32 {
                        continue;
                    }
                    if vis[nx as usize][ny as usize] == 0 {
                        vis[nx as usize][ny as usize] = 1;
                        q.push_back( (nx as usize, ny as usize) );
                    }
                }
            }
            szs.push(cnt);
        }
    }
    szs.sort();
    let res = szs[szs.len() - 1] * szs[szs.len() - 2] * szs[szs.len() - 3];
    println!("{}", res);
}
