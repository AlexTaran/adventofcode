use std::io;

fn main() {
    let mut x = 1;
    let mut lp = 0;
    let mut sm = 0;
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
        lp += 1;
        if (lp - 20) % 40 == 0 && lp <= 220 {
            sm += x * lp;
        }
        if parts[0] == "addx" {
            lp += 1;
            if (lp - 20) % 40 == 0 && lp <= 220 {
                sm += x * lp;
            }
            x += parts[1].parse::<i32>().unwrap();
        }
    }
    println!("{}", sm);
}
