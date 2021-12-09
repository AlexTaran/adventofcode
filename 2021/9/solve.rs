use std::io;

fn main() {
    let mut arr:Vec<Vec<u32>> = Vec::new();

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().len() == 0 {
            break;
        }
        let v:Vec<u32> = line.trim().chars().map(|x|x.to_string().parse::<u32>().unwrap()).collect();
        arr.push(v);
    }
    let mut cnt = 0;
    let dx:Vec<i32> = vec!(-1,  0, 1, 0);
    let dy:Vec<i32> = vec!( 0, -1, 0, 1);
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            let mut lower = true;
            for d in 0..4 {
                let x = i as i32 + dx[d];
                let y = j as i32 + dy[d];
                if x >= 0 && y >= 0 && x < arr.len() as i32 && y < arr[i].len() as i32 {
                    if arr[x as usize][y as usize] <= arr[i][j] {
                        lower = false;
                        break;
                    }
                }
            }
            if lower {
                cnt += 1 + arr[i][j];
            }
        }
    }
    println!("{}", cnt);
}
