use std::io;

fn main() {
    let mut timesline = String::new();
    io::stdin().read_line(&mut timesline).expect("");
    let times:Vec<u64> = timesline[timesline.find(':').unwrap()+1..].trim().split_whitespace().map(|x|x.parse::<u64>().unwrap()).collect();

    let mut distline = String::new();
    io::stdin().read_line(&mut distline).expect("");
    let dists:Vec<u64> = distline[distline.find(':').unwrap()+1..].trim().split_whitespace().map(|x|x.parse::<u64>().unwrap()).collect();
    
    let mut res:u64 = 1;
    for i in 0..times.len() {
        let mut k = 0;
        for w in 0..times[i]+1 {
            let dist = (times[i] - w) * w;
            if dist > dists[i] {
                k += 1;
            }
        }
        res *= k;
    }
    println!("{}", res);

}
