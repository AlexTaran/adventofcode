use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let data:Vec<u64> = line.trim().split(",").map(|x|x.parse::<u64>().unwrap()).collect();
    let mut counters:Vec<u64> = vec![0;9];
    for d in data {
        counters[d as usize] += 1;
    }

    for _i in 0..256 {
        let mut new_counters:Vec<u64> = vec![0;9];
        new_counters[8] += counters[0];
        new_counters[6] += counters[0];
        for i in 1..9 {
            new_counters[i-1] += counters[i];
        }
        counters = new_counters;
    }

    let mut sm:u64 = 0;
    for i in 0..9 {
        sm += counters[i];
    }
    println!("{}", sm);
}
