use std::io;

fn main() {
    let mut sum = 0;

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().len() == 0 {
            break;
        }

        let parts:Vec<&str> = line.trim().split(" | ").collect();
        let one = parts[0].trim().split(" ").filter(|s|s.len() == 2).collect::<Vec<&str>>()[0];
        let sev = parts[0].trim().split(" ").filter(|s|s.len() == 3).collect::<Vec<&str>>()[0];
        let fou = parts[0].trim().split(" ").filter(|s|s.len() == 4).collect::<Vec<&str>>()[0];
        let mut number = 0;
        for s in parts[1].split(" ") {
            let mut digit = 0;
            if s.len() == 2 {
                digit = 1;
            } else if s.len() == 3 {
                digit = 7;
            } else if s.len() == 4 {
                digit = 4;
            } else if s.len() == 7 {
                digit = 8;
            } else if s.len() == 6 { // 9 6 0
                if s.find(fou.chars().nth(0).unwrap()) != None && s.find(fou.chars().nth(1).unwrap()) != None && s.find(fou.chars().nth(2).unwrap()) != None && s.find(fou.chars().nth(3).unwrap()) != None {
                    digit = 9;
                } else if s.find(sev.chars().nth(0).unwrap()) != None && s.find(sev.chars().nth(1).unwrap()) != None && s.find(sev.chars().nth(2).unwrap()) != None {
                    digit = 0;
                } else {
                    digit = 6;
                }
            } else if s.len() == 5 { // 5 2 3
                if s.find(one.chars().nth(0).unwrap()) != None && s.find(one.chars().nth(1).unwrap()) != None {
                    digit = 3;
                } else {
                    let mut sovp = 0;
                    for i in 0..4 {
                        if s.find(fou.chars().nth(i).unwrap()) != None {
                            sovp += 1;
                        }
                    }
                    if sovp == 3 {
                        digit = 5;
                    } else {
                        digit = 2;
                    }
                }
            }
            number *= 10;
            number += digit;
        }
        sum += number;
    }
    println!("{}", sum);
}
