use std::io;

fn main() {
    let mut seedline = String::new();
    io::stdin().read_line(&mut seedline).expect("");
    let mut numbers:Vec<u64> = seedline[seedline.find(':').unwrap()+1..].trim().split(" ").map(|x|x.parse::<u64>().unwrap()).collect();

    io::stdin().read_line(&mut seedline).expect(""); // skip empty line

    loop {
        let mut header = String::new();
        let num_bytes = io::stdin().read_line(&mut header).expect("");
        if num_bytes == 0 {
            break;
        }
        if header.find(':').is_some() {
            let mut dst:Vec<u64> = Vec::new();
            let mut src:Vec<u64> = Vec::new();
            let mut rng:Vec<u64> = Vec::new();
            loop {
                let mut line = String::new();
                let num_bytes = io::stdin().read_line(&mut line).expect("");
                if num_bytes == 0 || line.trim().len() == 0 {
                    break;
                }
                let parts:Vec<u64> = line.split_whitespace().map(|x|x.parse::<u64>().unwrap()).collect();
                dst.push(parts[0]);
                src.push(parts[1]);
                rng.push(parts[2]);
            }
            for i in 0..numbers.len() {
                for mpid in 0..src.len() {
                    if src[mpid] <= numbers[i] && numbers[i] < src[mpid] + rng[mpid] {
                        numbers[i] = numbers[i] - src[mpid] + dst[mpid];
                        break;
                    }
                }
            }
        }
    }
    println!("{}", numbers.iter().min().unwrap());
}
