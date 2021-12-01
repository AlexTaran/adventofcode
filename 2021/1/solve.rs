use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("");
    let mut prev:u32 = line.trim().parse().expect("");
    
    let mut count = 0;
    loop {
        line = String::new();
        io::stdin().read_line(&mut line).expect("");
        if line.trim().len() == 0 {
            break;
        }
        let curr:u32 = line.trim().parse().expect("");
        if prev < curr {
            count = count + 1;
        }
        prev = curr;
    }
    println!("{}", count);
}
