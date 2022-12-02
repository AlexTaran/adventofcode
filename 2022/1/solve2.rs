use std::io;

fn main() {
    let mut accum:u32 = 0;
    let mut vals = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        if line.trim().len() == 0 {
            vals.push(accum);
            accum = 0;
            continue;
        }
        let curr:u32 = line.trim().parse().expect("");
        accum += curr
    }
    vals.push(accum);
    vals.sort();
    vals.reverse();
    println!("{}", vals[0] + vals[1] + vals[2]);
}
