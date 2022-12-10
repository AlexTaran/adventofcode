use std::io;

fn main() {
    let mut x:i32 = 1;
    let mut lp:i32 = 0;
    let mut rows:Vec<String> = Vec::new();
    rows.push(String::new());
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        if line.trim().len() == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split(" ").collect();
        rows.last_mut().unwrap().push(if (x-lp%40).abs() <= 1 {'#'} else {'.'});
        lp += 1;
        if lp % 40 == 0 && lp <= 220 {
            rows.push(String::new());
        }
        if parts[0] == "addx" {
            rows.last_mut().unwrap().push(if (x-lp%40).abs() <= 1 {'#'} else {'.'});
            lp += 1;
            if lp % 40 == 0 && lp <= 220 {
                rows.push(String::new());
            }
            x += parts[1].parse::<i32>().unwrap();
        }
    }
    for line in rows {
        println!("{}", line);
    }
}
