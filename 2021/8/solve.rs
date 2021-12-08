use std::io;

fn main() {
    let mut cnt = 0;

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().len() == 0 {
            break;
        }

        let parts:Vec<&str> = line.trim().split(" | ").collect();
        for s in parts[1].split(" ") {
            if s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7 {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
