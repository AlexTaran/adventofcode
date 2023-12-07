use std::io;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum Comb {
  HighCard,
  Pair,
  TwoPairs,
  Three,
  FullHouse,
  Four,
  Five,
}

fn calc_comb(a: &String) -> Comb {
    let mut cnt: HashMap<char, u32> = HashMap::new();
    for c in a.chars() {
        *cnt.entry(c).or_insert(0) += 1;
    }
    if cnt.contains_key(&'J') {
        let cards = vec!['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
        return cards.iter().map(|ch|calc_comb(&a.to_string().replacen('J', &ch.to_string(), 1))).max().unwrap();
    }
    let vs: HashSet<u32> = cnt.values().cloned().collect();
    if vs.contains(&5) {
        return Comb::Five;
    }
    if vs.contains(&4) {
        return Comb::Four;
    }
    if vs.contains(&3) {
        if vs.contains(&2) {
            return Comb::FullHouse;
        }
        return Comb::Three;
    }
    if vs.contains(&2) {
        if cnt.len() == 3 {
            return Comb::TwoPairs;
        }
        return Comb::Pair;
    }
    return Comb::HighCard;
}

fn modify_lex(h: &String) -> String {
    return h.replace('T', "B").replace('J', "1").replace('Q', "D").replace('K', "E").replace('A', "F");
}

fn compare(a: &String, b: &String) -> Ordering {
    let ca = calc_comb(a);
    let cb = calc_comb(b);

    if ca.cmp(&cb) == Ordering::Equal {
        return modify_lex(a).cmp(&modify_lex(b));
    }
    return ca.cmp(&cb);
}

fn main() {
    let mut sum:u64 = 0;
    let mut hands:Vec<(String, u32)> = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split(" ").collect();
        hands.push((parts[0].to_string(), parts[1].parse::<u32>().unwrap()));
    }
    hands.sort_by(|p1, p2| compare(&p1.0, &p2.0));
    for i in 0..hands.len() {
        sum += (i+1) as u64 * hands[i].1 as u64;
    }
    println!("{}", sum);
}
