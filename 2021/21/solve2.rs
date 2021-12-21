use std::io;
use std::cmp;
use std::collections::HashMap;

fn wrappos(p: u32) -> u32 {
    let mut res = p;
    while res > 10 {
        res -= 10;
    }
    return res;
}

fn step(pos: &(u32, u32, u32, u32, bool)) -> Vec<(u32, u32, u32, u32, bool)> {
    let mut res = Vec::new();
    for d1 in 1..4 {
        for d2 in 1..4 {
            for d3 in 1..4 {
                let sm = d1 + d2 + d3;
                if !pos.4 {
                    let nxtps = wrappos(pos.0 + sm);
                    res.push( (nxtps, pos.1, pos.2+nxtps, pos.3, true) );
                } else {
                    let nxtps = wrappos(pos.1 + sm);
                    res.push( (pos.0, nxtps, pos.2, pos.3+nxtps, false) );
                }
            }
        }
    }
    return res;
}

fn main() {
    let mut positions: [u32; 2] = [0, 0];
    for id in 0..2 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim()[28..].to_string();
        positions[id] = line.parse().unwrap();
    }
    let mut poscnt: HashMap<(u32, u32, u32, u32, bool), u64> = HashMap::new();
    poscnt.insert((positions[0], positions[1], 0, 0, false), 1);

    let mut wincnt: [u64; 2] = [0, 0];
    loop {
        if poscnt.len() == 0 {
            break;
        }
        let mut nxt: HashMap<(u32, u32, u32, u32, bool), u64> = HashMap::new();
        for (key, val) in poscnt.iter() {
            let stpvars = step(key);
            for stp in stpvars {
                if stp.2 >= 21 {
                    wincnt[0] += val;
                    continue;
                }
                if stp.3 >= 21 {
                    wincnt[1] += val;
                    continue;
                }
                if !nxt.contains_key(&stp) {
                    nxt.insert(stp, 0);
                }
                *nxt.get_mut(&stp).unwrap() += val;
            }
        }
        poscnt = nxt;
    }

    println!("{}", cmp::max(wincnt[0], wincnt[1]));

    /*
    let mut scores: [i32; 2] = [0, 0];
    let mut turn = 0;
    let mut nxtdie = 1;
    while scores[0] < 21 && scores[1] < 21 {
        for _i in 0..3 {
            positions[turn] += nxtdie;
            nxtdie += 1;
            if nxtdie > 100 {
                nxtdie -= 100;
            }
        }
        while positions[turn] > 10 {
            positions[turn] -= 10;
        }
        scores[turn] += positions[turn];
        turn = 1 - turn;
    }*/
}
