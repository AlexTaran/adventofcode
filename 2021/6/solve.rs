use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let mut data:Vec<u32> = line.trim().split(",").map(|x|x.parse::<u32>().unwrap()).collect();

    for _i in 0..80 {
        let mut new_data:Vec<u32> = Vec::new();
        for t in data.iter() {
            if t > &0 {
                new_data.push(t-1);
            } else {
                new_data.push(6);
                new_data.push(8);
            }
        }
        data = new_data;
    }

    println!("{}", data.len());
}
