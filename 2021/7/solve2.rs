use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let data:Vec<u32> = line.trim().split(",").map(|x|x.parse::<u32>().unwrap()).collect();

    let mx = (data.iter().max().unwrap() + 1) as usize;
    let mut deltas = vec![0; mx];
    for i in 1..mx {
        deltas[i] = deltas[i-1] + i;
    }
    let mut cand = u32::MAX;
    for i in 0..mx {
        let mut sm:u32 = 0;
        for d in data.iter() {
            sm += deltas[(i as i32 - *d as i32).abs() as usize] as u32;
        }
        if sm < cand {
            cand = sm;
        }
    }
    println!("{}", cand);
}
