use std::io;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let mut ccount:HashMap<u32, u32> = HashMap::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split(": ").collect();
        let cnparts:Vec<&str> = parts[0].split_whitespace().collect();
        let card_number = cnparts[1].parse::<u32>().unwrap();
        *ccount.entry(card_number).or_insert(0) += 1;

        let curcount = *ccount.get(&card_number).unwrap();

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
        for nxt in (card_number+1)..(card_number+1+count) {
          *ccount.entry(nxt).or_insert(0) += curcount;
        }
    }
    println!("{}", ccount.values().sum::<u32>());
}
