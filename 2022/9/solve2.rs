use std::io;
use std::collections::HashSet;

fn main() {
    let mut x:Vec<i32> = vec![0; 10];
    let mut y:Vec<i32> = vec![0; 10];
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
                x[0] += 1;
            } else if parts[0] == "L" {
                x[0] -= 1;
            } else if parts[0] == "U" {
                y[0] += 1;
            } else if parts[0] == "D" {
                y[0] -= 1;
            }
            for j in 1..10 {
                let dx = (x[j-1] - x[j]).abs();
                let dy = (y[j-1] - y[j]).abs();
                if dx <= 1 && dy <= 1 {
                    continue;
                }
                if dx == 0 && dy == 2 {
                    y[j] += (y[j-1] - y[j]) / 2;
                } else if dx == 2 && dy == 0 {
                    x[j] += (x[j-1] - x[j]) / 2;
                } else if dx == 1 && dy == 2 {
                    x[j] = x[j-1];
                    y[j] += (y[j-1] - y[j]) / 2;
                } else if dx == 2 && dy == 1 {
                    y[j] = y[j-1];
                    x[j] += (x[j-1] - x[j]) / 2;
                } else if dx == 2 && dy == 2 {
                    y[j] += (y[j-1] - y[j]) / 2;
                    x[j] += (x[j-1] - x[j]) / 2;
                } else {
                    println!("some fail");
                }
            }
            visited.insert( (x[9], y[9]) );
        }
    }
    println!("{}", visited.len());
}
