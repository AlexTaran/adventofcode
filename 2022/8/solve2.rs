use std::io;

fn main() {
    let mut table = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        if line.trim().len() == 0 {
            break;
        }
        let parts:Vec<u32> = line.trim().chars().map(|x|x.to_string().parse::<u32>().unwrap()).collect();
        table.push(parts);
    }
    let mut mx = 0;
    for i in 0..table.len() {
        for j in 0..table[i].len() {
            let cur = table[i][j];
            let mut koef = 1;
            let mut r = true;
            for k in j+1..table[i].len() {
                if table[i][k] >= cur {
                    koef *= k - j ;
                    r = false;
                    break;
                }
            }
            if r {
                koef *= table[i].len() - 1 - j;
            }
            r = true;
            for k in (0..j).rev() {
                if table[i][k] >= cur {
                    koef *= j - k ;
                    r = false;
                    break;
                }
            }
            if r {
                koef *= j;
            }
            r = true;
            for k in i+1..table.len() {
                if table[k][j] >= cur {
                    koef *= k - i ;
                    r = false;
                    break;
                }
            }
            if r {
                koef *= table.len() - 1 - i;
            }
            r = true;
            for k in (0..i).rev() {
                if table[k][j] >= cur {
                    koef *= i - k ;
                    r = false;
                    break;
                }
            }
            if r {
                koef *= i;
            }
            if koef > mx {
                mx = koef;
            }
        }
    }
    println!("{}", mx);
}
