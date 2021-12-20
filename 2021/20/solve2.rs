use std::io;

fn wrap(data: &Vec<Vec<bool>>, sz: usize) -> Vec<Vec<bool>> {
    let mut res = Vec::new();
    let width = data[0].len() + sz * 2;
    for _i in 0..sz {
        res.push(vec![false; width]);
    }
    for d in data.iter() {
        let mut v = Vec::new();
        for _i in 0..sz {
            v.push(false);
        }
        for b in d.iter() {
            v.push(*b);
        }
        for _i in 0..sz {
            v.push(false);
        }
        res.push(v);
    }
    for _i in 0..sz {
        res.push(vec![false; width]);
    }
    return res;
}

fn step(data: &Vec<Vec<bool>>, enh: &str) -> Vec<Vec<bool>> {
    let mut res = Vec::new();
    let inf:bool = data[0][0];
    for i in 0..data.len() {
        let mut line = Vec::new();
        for j in 0..data[i].len() {
            let mut coord = 0;
            for di in 0..3 {
                for dj in 0..3 {
                    let mut ci:i32 = (i+di) as i32;
                    let mut cj:i32 = (j+dj) as i32;
                    ci -= 1;
                    cj -= 1;
                    let mut g = inf;
                    if ci >= 0 && cj >= 0 && ci < data.len() as i32 && cj < data[i].len() as i32 {
                        g = data[ci as usize][cj as usize];
                    }
                    coord *= 2;
                    if g {
                        coord += 1;
                    }
                }
            }
            let val = enh.chars().nth(coord).unwrap();
            line.push(if val == '#' { true } else { false });
        }
        res.push(line);
    }
    return res;
}

fn count(data: &Vec<Vec<bool>>) -> u32 {
    let mut res = 0;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] {
                res += 1;
            }
        }
    }
    return res;
}

fn main() {
    let mut enh = String::new();
    io::stdin().read_line(&mut enh).unwrap();

    let mut inp:Vec<Vec<bool>> = Vec::new();
    io::stdin().read_line(&mut String::new()).unwrap();
	loop {                                                             
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.len() == 0 {
            break;
        }
        let data:Vec<bool> = line.trim().chars().map(|x| if x=='#' { true } else { false }).collect();
        inp.push(data);
	}

    inp = wrap(&inp, 55);
    for _i in 0..50 {
        inp = step(&inp, &enh);
    }

    println!("{}", count(&inp));
}
