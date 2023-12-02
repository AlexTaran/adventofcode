use std::io;

fn main() {
    let mut sum:u32 = 0;
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split(": ").collect();
        let game_number = &parts[0][5..].parse::<u32>().unwrap();

        let mut fail = false;

        for color_group in parts[1].split("; ") {
          for color in color_group.split(", ") {
            let colparts:Vec<&str> = color.split(" ").collect();
            let cnt = colparts[0].parse::<u32>().unwrap();
            if colparts[1] == "red" && cnt > 12 {
              fail = true
            }
            if colparts[1] == "green" && cnt > 13 {
              fail = true
            }
            if colparts[1] == "blue" && cnt > 14 {
              fail = true
            }
          }
        }
        if !fail {
          sum += game_number;
        }
    }
    println!("{}", sum);
}
