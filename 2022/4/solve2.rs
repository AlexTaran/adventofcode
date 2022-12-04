use std::io;

fn main() {
    let mut sm:u32 = 0;
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        if line.trim().len() == 0 {
            break;
        }
        let ranges:Vec<&str> = line.trim().split(",").collect();
        let r0:Vec<u32> = ranges[0].split("-").map(|x|x.parse::<u32>().unwrap()).collect();
        let r1:Vec<u32> = ranges[1].split("-").map(|x|x.parse::<u32>().unwrap()).collect();
        if r0[1] < r1[0] || r1[1] < r0[0] {
            continue;
        }
        sm += 1;
    }
    println!("{}", sm);
}
