use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("");
    for i in 14..line.len() {
        let vec:Vec<char> = line.chars().collect::<Vec<char>>();
        let mut sub = vec[(i-14)..i].to_vec();
        sub.sort();
        let mut fail = false;
        for j in 1..14 {
            if sub[j-1] == sub[j] {
                fail = true;
                break;
            }
        }
        if !fail {
            println!("{}", i);
            break;
        }
    }
}
