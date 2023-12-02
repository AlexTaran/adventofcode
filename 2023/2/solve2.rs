use std::io;
use std::cmp;

fn main() {
    let mut sum:u32 = 0;
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split(": ").collect();

        let mut rmin = 0;
        let mut gmin = 0;
        let mut bmin = 0;

        for color_group in parts[1].split("; ") {
          for color in color_group.split(", ") {
            let colparts:Vec<&str> = color.split(" ").collect();
            let cnt = colparts[0].parse::<u32>().unwrap();
            if colparts[1] == "red" {
              rmin = cmp::max(rmin, cnt);
            }
            if colparts[1] == "green" {
              gmin = cmp::max(gmin, cnt);
            }
            if colparts[1] == "blue" {
              bmin = cmp::max(bmin, cnt);
            }
          }
        }
        sum += rmin*gmin*bmin;
    }
    println!("{}", sum);
}
