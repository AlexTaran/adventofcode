use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("");
    let mut prev1:u32 = line.trim().parse().expect("");

    line = String::new();
    io::stdin().read_line(&mut line).expect("");
    let mut prev2 = line.trim().parse::<u32>().expect("");
    
    line = String::new();
    io::stdin().read_line(&mut line).expect("");
    let mut prev3 = line.trim().parse::<u32>().expect("");

    let mut count = 0;
    loop {
        line = String::new();
        io::stdin().read_line(&mut line).expect("");
        if line.trim().len() == 0 {
            break;
        }
        let curr:u32 = line.trim().parse().expect("");
        if prev1 + prev2 + prev3 < prev2 + prev3 + curr {
            count = count + 1;
        }
        prev1 = prev2;
        prev2 = prev3;
        prev3 = curr;
    }
    println!("{}", count);
}
