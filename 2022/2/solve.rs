use std::io;

fn main() {
    let mut sm:u32 = 0;
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

        if parts[1] == "X" {
          sm += 1;
          if parts[0] == "A" {
            sm += 3;
          } else if parts[0] == "B" {
            sm += 0;
          } else if parts[0] == "C" {
            sm += 6;
          }
        } else if parts[1] == "Y" {
          sm += 2;
          if parts[0] == "A" {
            sm += 6;
          } else if parts[0] == "B" {
            sm += 3;
          } else if parts[0] == "C" {
            sm += 0;
          }
        } else if parts[1] == "Z" {
          sm += 3;
          if parts[0] == "A" {
            sm += 0;
          } else if parts[0] == "B" {
            sm += 6;
          } else if parts[0] == "C" {
            sm += 3;
          }
        }
    }
    println!("{}", sm);
}
