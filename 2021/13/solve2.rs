use std::io;
use std::cmp;
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
    }
    let mut mx = 0;
    let mut my = 0;
    for d in dots.iter() {
        mx = cmp::max(mx, d.0);
        my = cmp::max(my, d.1);
    }
    for y in 0..(my+1) {
        for x in 0..(mx+1) {
            if dots.contains( &(x, y) ) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    print!("{} {}", mx, my);
}
