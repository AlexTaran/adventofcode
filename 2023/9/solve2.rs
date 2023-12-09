use std::io;

fn main() {
    let mut sum:i32 = 0;
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let seq:Vec<i32> = line.split_whitespace().map(|x|x.parse::<i32>().unwrap()).collect();
        let mut arrs:Vec<Vec<i32>> = Vec::new();
        arrs.push(seq);
        loop {
            let lst:&Vec<i32> = &arrs.last().unwrap();
            let diff:Vec<i32> = (1..lst.len()).map(|i|lst[i] - lst[i-1]).collect();
            if diff.iter().all(|&x|x==0) {
                break;
            }
            arrs.push(diff);
        }
        let mut elem = 0;
        for i in (0..arrs.len()).rev() {
            elem = arrs[i].first().unwrap() - elem;
        }
        sum += elem;
    }
    println!("{}", sum);
}
