use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let elems:Vec<u32> = line.trim().split(",").map(|x|x.parse::<u32>().unwrap()).collect();
    io::stdin().read_line(&mut line).unwrap(); // skip

    let mut tables:Vec<Vec<Vec<u32>>> = Vec::new();
    let mut marks:Vec<Vec<Vec<bool>>> = Vec::new();

    loop {
        line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().len() == 0 {
            break;
        }
        let mut table:Vec<Vec<u32>> = Vec::new();
        let mut mks:Vec<Vec<bool>> = Vec::new();
        for _i in [0,1,2,3,4] {
            table.push(line.trim().split(" ").filter(|x|x.len() > 0).map(|x|x.parse::<u32>().unwrap()).collect());
            mks.push(vec![false,false,false,false,false]);
            line = String::new();
            io::stdin().read_line(&mut line).unwrap();
        }
        marks.push(mks);
        tables.push(table);
    }


    let mut wins:Vec<bool> = Vec::new();
    wins.resize(tables.len(), false);
    let mut lwidx:i32 = -1;
    for x in elems.iter() {
        for (ti, t) in tables.iter().enumerate() {
            for i in [0,1,2,3,4] {
                for j in [0,1,2,3,4] {
                    if t[i][j] == *x {
                        marks[ti][i][j] = true;
                    }
                }
            }
            // check
            for i in [0,1,2,3,4] {
                let mut f1 = true;
                let mut f2 = true;
                for j in [0,1,2,3,4] {
                    if !marks[ti][i][j] {
                        f1 = false;
                    }
                    if !marks[ti][j][i] {
                        f2 = false;
                    }
                }
                if f1 || f2 {
                    wins[ti] = true;
                }
            }
        }
        // check if last
        if lwidx == -1 {
            for (i, w) in wins.iter().enumerate() {
                if *w == false {
                    if lwidx >= 0 {
                        lwidx = -1;
                        break;
                    }
                    lwidx = i as i32;
                }
            }
        }
        if lwidx >= 0 && wins[lwidx as usize] {
            let mut sm:u32 = 0;
            for mi in [0,1,2,3,4] {
                for mj in [0,1,2,3,4] {
                    if !marks[lwidx as usize][mi][mj] {
                        sm += tables[lwidx as usize][mi][mj];
                    }
                }
            }
            println!("{}", sm * x);
            return;
        }
    }
}
