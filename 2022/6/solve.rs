use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("");
    for i in 4..line.len() {
        let vec:Vec<char> = line.chars().collect::<Vec<char>>();
        let mut sub = vec[(i-4)..i].to_vec();
        sub.sort();
        if sub[0] == sub[1] || sub[1] == sub[2] || sub[2] == sub[3] {
            continue;
        }
        println!("{}", i);
        break;
    }
}
