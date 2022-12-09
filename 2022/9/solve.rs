use std::io;
use std::collections::HashSet;

fn main() {
    let mut xh:i32 = 0;
    let mut yh:i32 = 0;
    let mut xt:i32 = 0;
    let mut yt:i32 = 0;
    let mut visited = HashSet::new();
    visited.insert( (0, 0) );
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        if line.trim().len() == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split(" ").collect();
        let dist = parts[1].parse::<u32>().unwrap();
        for _i in 0..dist {
            if parts[0] == "R" {
                xh += 1;
            } else if parts[0] == "L" {
                xh -= 1;
            } else if parts[0] == "U" {
                yh += 1;
            } else if parts[0] == "D" {
                yh -= 1;
            }
            let dx = (xh - xt).abs();
            let dy = (yh - yt).abs();
            if dx <= 1 && dy <= 1 {
                continue;
            }
            if dx == 0 && dy == 2 {
                yt += (yh - yt) / 2;
            } else if dx == 2 && dy == 0 {
                xt += (xh - xt) / 2;
            } else if dx == 1 && dy == 2 {
                xt = xh;
                yt += (yh - yt) / 2;
            } else if dx == 2 && dy == 1 {
                yt = yh;
                xt += (xh - xt) / 2;
            } else {
                println!("some fail {} {}", dx, dy);
            }
            visited.insert( (xt, yt) );
        }
    }
    println!("{}", visited.len());
}
