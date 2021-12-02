use std::io;

fn main() {
    
    let mut x = 0;
    let mut y = 0;

    let mut aim = 0;
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("");
        if line.trim().len() == 0 {
            break;
        }
        let spl:Vec<&str> = line.trim().split(" ").collect();
        let direction = spl[0];
        let delta:u32 = spl[1].parse().expect("");
        if direction == "forward" {
            x += delta;
            y += aim * delta;
        }
        if direction == "down" {
            aim += delta;
        }
        if direction == "up" {
            aim -= delta;
        }
    }
    println!("{}", x * y);
}
