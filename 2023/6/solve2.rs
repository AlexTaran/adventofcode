use std::io;

fn main() {
    let mut timesline = String::new();
    io::stdin().read_line(&mut timesline).expect("");
    let time:u64 = timesline[timesline.find(':').unwrap()+1..].trim().split_whitespace().collect::<Vec<&str>>().join("").parse::<u64>().unwrap();

    
    let mut distline = String::new();
    io::stdin().read_line(&mut distline).expect("");
    let dist:u64 = distline[distline.find(':').unwrap()+1..].trim().split_whitespace().collect::<Vec<&str>>().join("").parse::<u64>().unwrap();
    
    let mut res:u64 = 0;
    for w in 0..time+1 {
        let d = (time - w) * w;
        if d > dist {
            res += 1;
        }
    }
    println!("{}", res);
}
