use std::io;

fn main() {
    let mut table = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        if line.trim().len() == 0 {
            break;
        }
        let parts:Vec<u32> = line.trim().chars().map(|x|x.to_string().parse::<u32>().unwrap()).collect();
        table.push(parts);
    }
    let mut cnt = 0;
    for i in 0..table.len() {
        for j in 0..table[i].len() {
            let cur = table[i][j];
            let mut vis = true;
            for k in j+1..table[i].len() {
                if table[i][k] >= cur {
                    vis = false;
                    break;
                }
            }
            if vis {
                cnt += 1;
                continue;
            }
            vis = true;
            for k in 0..j {
                if table[i][k] >= cur {
                    vis = false;
                    break;
                }
            }
            if vis {
                cnt += 1;
                continue;
            }
            vis = true;
            for k in i+1..table.len() {
                if table[k][j] >= cur {
                    vis = false;
                    break;
                }
            }
            if vis {
                cnt += 1;
                continue;
            }
            vis = true;
            for k in 0..i {
                if table[k][j] >= cur {
                    vis = false;
                    break;
                }
            }
            if vis {
                cnt += 1;
                continue;
            }
        }
    }
    println!("{}", cnt);
}
