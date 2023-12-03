use std::io;
use std::collections::HashMap;

fn main() {
    let mut sum:u64 = 0;
    let mut field:Vec<Vec<char>> = Vec::new();
    let mut gears: HashMap<(i32, i32), Vec<u32>> = HashMap::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let chars:Vec<char> = line.trim().chars().collect();
        field.push(chars);
    }
    let is_gear = |y: i32, x: i32| -> bool {
        if y < 0 || x < 0 || y >= field.len() as i32 || x >= field[0].len() as i32 {
            return false;
        }
        let ch = field[y as usize][x as usize];
        return ch == '*';
    };
    let is_neib_gear = |y:i32, x:i32, rx: i32| -> (bool, i32, i32) {
        if is_gear(y, x-1) {
            return (true, y, x-1);
        }
        if is_gear(y, rx) {
            return (true, y, rx);
        }
        for ix in (x-1)..(rx+1) {
            if is_gear(y-1, ix) {
                return (true, y-1, ix);
            }
            if is_gear(y+1, ix) {
                return (true, y+1, ix);
            }
        }
        return (false, 0, 0);
    };
    for y in 0..field.len() {
        let mut x:i32 = 0;
        while x < field[0].len() as i32 {
            if field[y][x as usize].is_digit(10) {
                let mut val = 0;
                let mut rx:i32 = x;
                while rx < field[0].len() as i32 && field[y][rx as usize].is_digit(10) {
                    val = val*10+field[y][rx as usize].to_digit(10).unwrap();
                    rx += 1;
                }
                let res = is_neib_gear(y as i32, x, rx);
                if res.0 {
                    let coord = (res.1, res.2);
                    gears.entry(coord).or_insert(vec![]).push(val);
                }
                x = rx;
            }
            x += 1;
        }
    }

    for v in gears.values() {
        if v.len() == 2 {
            sum += v[0] as u64 * v[1] as u64;
        }
    }
    println!("{}", sum);
}
