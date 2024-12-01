use std::io;
use std::collections::HashMap;

fn main() {
    let mut sum:i32 = 0;
    let mut vx:Vec<i32> = Vec::new();
    let mut m:HashMap<i32, i32> = HashMap::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split(" ").collect();
        let x = parts[0].parse::<i32>().unwrap();
        let y = parts[parts.len()-1].parse::<i32>().unwrap();
        vx.push(x);
        if m.contains_key(&y) {
            *m.get_mut(&y).unwrap() += 1;
        } else {
            m.insert(y, 1);
        }
    }
    for idx in 0..vx.len() {
        if m.contains_key(&vx[idx]) {
            sum += vx[idx] * *m.get(&vx[idx]).unwrap()
        }
    }
    println!("{}", sum);
}
