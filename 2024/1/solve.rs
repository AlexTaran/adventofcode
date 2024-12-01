use std::io;

fn main() {
    let mut sum:i32 = 0;
    let mut vx:Vec<i32> = Vec::new();
    let mut vy:Vec<i32> = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split(" ").collect();
        let x = parts[0].parse::<i32>().unwrap();
        let y = parts[parts.len()-1].parse::<i32>().unwrap();
        vx.push(x);
        vy.push(y);
    }
    vx.sort();
    vy.sort();
    for idx in 0..vx.len() {
        sum += (vx[idx] - vy[idx]).abs();
    }
    println!("{}", sum);
}
