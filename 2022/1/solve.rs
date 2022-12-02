use std::io;

fn main() {
    let mut accum:u32 = 0;
    let mut maxsum:u32 = 0;
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        if line.trim().len() == 0 {
            if accum > maxsum {
                maxsum = accum;
            }
            accum = 0;
            continue;
        }
        let curr:u32 = line.trim().parse().expect("");
        accum += curr
    }
    if accum > maxsum {
        maxsum = accum;
    }
    println!("{}", maxsum);
}
