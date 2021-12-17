use std::io;
use std::cmp;

fn calc_y_max(vx: i32, vy: i32, xmin: i32, xmax: i32, ymin: i32, ymax: i32) -> (i32, bool) {
    let mut yfound = 0;
    let mut x = 0;
    let mut y = 0;
    let mut vvx = vx;
    let mut vvy = vy;
    loop {
        x += vvx;
        y += vvy;
        yfound = cmp::max(yfound, y);
        vvy -= 1;
        if vvx > 0 {
            vvx -= 1;
        }
        if y < ymin {
            break;
        }
        if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
            return (yfound, true);
        }
    }
    return (yfound, false);
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line = line.trim()[15..].to_string();
    let parts:Vec<&str> = line.split(", y=").collect();
    let xs:Vec<&str> = parts[0].split("..").collect();
    let ys:Vec<&str> = parts[1].split("..").collect();
    let xmin = xs[0].parse::<i32>().unwrap();
    let xmax = xs[1].parse::<i32>().unwrap();
    let ymin = ys[0].parse::<i32>().unwrap();
    let ymax = ys[1].parse::<i32>().unwrap();

    let mut yfound = 0;
    for vx in 0..(xmax+1) {
        for vy in 0..200 {
            let (y, fl) = calc_y_max(vx, vy, xmin, xmax, ymin, ymax);
            if fl {
                yfound = cmp::max(yfound, y);
            }
        }
    }
    println!("{}", yfound);
}
