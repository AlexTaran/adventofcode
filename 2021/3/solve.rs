use std::io;

fn main() {
    let mut z = Vec::new();
    let mut o = Vec::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if z.len() == 0 {
            z.resize(line.trim().len(), 0);
            o.resize(line.trim().len(), 0);
        }
        if line.trim().len() == 0 {
            break;
        }
        for (i, c) in line.trim().chars().enumerate() {
            if c == '0' {
                z[i] += 1;
            }
            if c == '1' {
                o[i] += 1
            }
        }
    }
    let mut gv = Vec::new();
    let mut ev = Vec::new();
    for (i, x) in z.iter().enumerate() {
        if x > &o[i] {
            gv.push("1");
            ev.push("0");
        } else {
            gv.push("0");
            ev.push("1");
        }
    }
    let g = u32::from_str_radix(&gv.join(""), 2).unwrap();
    let e = u32::from_str_radix(&ev.join(""), 2).unwrap();
    println!("{}", g * e);
}
