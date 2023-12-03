use std::io;

fn main() {
    let mut sum:u32 = 0;
    let mut field:Vec<Vec<char>> = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let chars:Vec<char> = line.trim().chars().collect();
        field.push(chars);
    }
    let is_spec = |y: i32, x: i32| -> bool {
        if y < 0 || x < 0 || y >= field.len() as i32 || x >= field[0].len() as i32 {
            return false;
        }
        let ch = field[y as usize][x as usize];
        return !ch.is_digit(10) && ch != '.';
    };
    let is_neib_spec = |y:i32, x:i32, rx: i32| -> bool {
        if is_spec(y, x-1) || is_spec(y, rx) {
            return true;
        }
        for ix in (x-1)..(rx+1) {
            if is_spec(y-1, ix) || is_spec(y+1, ix) {
                return true;
            }
        }
        return false;
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
                if is_neib_spec(y as i32, x, rx) {
                    sum += val;
                }
                x = rx;
            }
            x += 1;
        }
    }
    println!("{}", sum);
}
