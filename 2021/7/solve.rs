use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let mut data:Vec<u32> = line.trim().split(",").map(|x|x.parse::<u32>().unwrap()).collect();

    data.sort();
    let md = data[data.len() / 2]; // In our input both candidates for median are equal.
    let mut sm = 0;
    for d in data.iter() {
        sm += (md as i32 - *d as i32).abs();
    }
    println!("{}", sm);
}
