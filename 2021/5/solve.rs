use std::io;
use std::cmp;

fn main() {
    let mut field:Vec<Vec<u32>> = Vec::new();
    for _i in 0..1000 {
        field.push(vec![0; 1000]);
    }

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().len() == 0 {
            break;
        }
        let spl:Vec<&str> = line.trim().split(" -> ").collect();
        let p1:Vec<u32> = spl[0].trim().split(",").map(|x|x.parse::<u32>().unwrap()).collect();
        let p2:Vec<u32> = spl[1].trim().split(",").map(|x|x.parse::<u32>().unwrap()).collect();
        if p1[0] == p2[0] {
            for x in cmp::min(p1[1], p2[1]) .. (cmp::max(p1[1], p2[1]) + 1) {
                field[p1[0] as usize][x as usize] += 1;
            }
        } else if p1[1] == p2[1] {
            for x in cmp::min(p1[0], p2[0]) .. (cmp::max(p1[0], p2[0]) + 1) {
                field[x as usize][p1[1] as usize] += 1;
            }
        }
    }

    let mut cnt:u32 = 0;

    for i in 0..1000 {
        for j in 0..1000 {
            if field[i][j] >= 2 {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}
