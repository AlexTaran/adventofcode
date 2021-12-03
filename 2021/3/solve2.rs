use std::io;

fn main() {
    let mut inp = Vec::<String>::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.trim().len() == 0 {
            break;
        }
        inp.push(line.trim().to_string());
    }

    let mut flt1:Vec::<String> = inp.iter().map(|x|x.clone()).collect();
    let mut idx = 0;
    while flt1.len() > 1 {
        let mut zc = 0;
        let mut oc = 0;
        for s in flt1.iter() {
            if s.chars().nth(idx).unwrap() == '1' {
                oc += 1;
            } else {
                zc += 1;
            }
        }
        let mut mc = '1';
        if zc > oc {
            mc = '0';
        }
        flt1 = flt1.into_iter().filter(|s|s.chars().nth(idx).unwrap() == mc).collect();
        idx += 1;
    }
    let or = u32::from_str_radix(&flt1[0], 2).unwrap();

    flt1 = inp.iter().map(|x|x.clone()).collect();
    idx = 0;
    while flt1.len() > 1 {
        let mut zc = 0;
        let mut oc = 0;
        for s in flt1.iter() {
            if s.chars().nth(idx).unwrap() == '1' {
                oc += 1;
            } else {
                zc += 1;
            }
        }
        let mut lc = '0';
        if zc > oc {
            lc = '1';
        }
        flt1 = flt1.into_iter().filter(|s|s.chars().nth(idx).unwrap() == lc).collect();
        idx += 1;
    }
    let cr = u32::from_str_radix(&flt1[0], 2).unwrap();
    println!("{}", or * cr);
}
