use std::io;
use std::collections::HashSet;

fn main() {
    let mut sum:u32 = 0;
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split(": ").collect();

        let card_nums:Vec<&str> = parts[1].split(" | ").collect();

        let mut card = HashSet::new();
        let mut count = 0;
        for ncard in card_nums[0].split_whitespace() {
            card.insert(ncard.parse::<u32>().unwrap());
        }
        for nhand in card_nums[1].split_whitespace() {
            let n = nhand.parse::<u32>().unwrap();
            if card.contains(&n) {
                count += 1;
            }
        }

        if count > 0 {
            sum += 2_u32.pow(count-1);
        }
    }
    println!("{}", sum);
}
