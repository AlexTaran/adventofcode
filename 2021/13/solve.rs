use std::io;
use std::collections::HashSet;

fn main() {
    let mut dots = HashSet::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().len() == 0 {
            break;
        }
        let elems:Vec<u32> = line.trim().split(",").map(|x|x.parse::<u32>().unwrap()).collect();
        dots.insert( (elems[0], elems[1]) );
    }
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().len() == 0 {
            break;
        }
        let spl:Vec<&str> = line.trim().split("=").collect();
        let axis = spl[0].chars().last().unwrap().to_string();
        let coord = spl[1].parse::<u32>().unwrap();
        let mut newdots = HashSet::new();
        for d in dots {
            if axis == "x" {
                if d.0 > coord {
                    newdots.insert((coord + coord - d.0, d.1));
                } else {
                    newdots.insert(d);
                }
            } else {
                if d.1 > coord {
                    newdots.insert((d.0, coord + coord - d.1));
                } else {
                    newdots.insert(d);
                }
            }
        }
        dots = newdots;
        break;
    }
    println!("{}", dots.len());
}
