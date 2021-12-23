use std::io;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;

#[derive(Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Pos {
    slots: [char; 7],
    cors: [[char; 2]; 4],
}

fn emp_pos() -> Pos {
    return Pos {slots: ['.'; 7], cors: [['.'; 2];4]};
}

fn is_final(p: &Pos) -> bool {
    return
        p.cors[0][0] == 'A' && p.cors[0][1] == 'A' &&
        p.cors[1][0] == 'B' && p.cors[1][1] == 'B' &&
        p.cors[2][0] == 'C' && p.cors[2][1] == 'C' &&
        p.cors[3][0] == 'D' && p.cors[3][1] == 'D';
}

fn dest_cor(ch: char) -> u32 {
    if ch == 'A' {
        return 0;
    } else if ch == 'B' {
        return 1;
    } else if ch == 'C' {
        return 2;
    } else if ch == 'D' {
        return 3;
    }
    return 0;
}

fn step_cost(ch: char) -> u32 {
    let mut res = 1;
    for _i in 0..dest_cor(ch) {
        res *= 10;
    }
    return res;
}

fn right_slot_from_cor(cor: u32) -> u32 {
    return cor + 2;
}

fn steps_to_cor(slot: u32, cor: u32) -> u32 {
    let mut steps = 0;
    let mut s = slot;
    if s == 0 {
        s += 1;
        steps += 1;
    }
    if s == 6 {
        s -= 1;
        steps += 1;
    }
    if s <= cor + 1 {
        steps += 2 * (cor + 1 - s);
    } else {
        steps += 2 * (s - cor - 2);
    }
    steps += 1;
    return steps;
}

fn neibs(p: &Pos) -> Vec<(Pos, u32)> {
    let mut res = Vec::new();

    for i in 0..7 { // move slots to cors
        if p.slots[i] == '.' {
            continue;
        }
        let cor = dest_cor(p.slots[i]);
        if p.cors[cor as usize][1] != '.' {
            continue;
        }
        let mut cango = true;
        if i < right_slot_from_cor(cor) as usize {
            for j in (i+1)..right_slot_from_cor(cor) as usize {
                if p.slots[j] != '.' {
                    cango = false;
                    break;
                }
            }
        } else {
            for j in right_slot_from_cor(cor) as usize..i {
                if p.slots[j] != '.' {
                    cango = false;
                    break;
                }
            }
        }
        if !cango {
            continue;
        }
        // Here ready to move!
        let mut steps = steps_to_cor(i as u32, cor);
        let mut np = p.clone();
        np.slots[i] = '.';
        if p.cors[cor as usize][0] == '.' {
            np.cors[cor as usize][0] = p.slots[i];
            steps += 2;
        } else {
            np.cors[cor as usize][1] = p.slots[i];
            steps += 1;
        }
        res.push((np, steps * step_cost(p.slots[i])));
    }
    for i in 0..4 { // move cors to slots
        if p.cors[i][0] == '.' && p.cors[i][1] == '.' {
            continue;
        }
        let mut ch = p.cors[i][1];
        let mut stp = 1;
        let mut np = p.clone();
        np.cors[i][1] = '.';
        if ch == '.' {
            ch = p.cors[i][0];
            stp += 1;
            np.cors[i][0] = '.';
        }
        let mut cur = right_slot_from_cor(i as u32);
        while cur < 7 && p.slots[cur as usize] == '.' {
            // going right
            let steps = stp + steps_to_cor(cur, i as u32);
            let mut nnp = np.clone();
            nnp.slots[cur as usize] = ch;
            res.push((nnp, steps * step_cost(ch)));
            cur += 1;
        }
        cur = right_slot_from_cor(i as u32) - 1;
        while p.slots[cur as usize] == '.' {
            // going left
            let steps = stp + steps_to_cor(cur, i as u32);
            let mut nnp = np.clone();
            nnp.slots[cur as usize] = ch;
            res.push((nnp, steps * step_cost(ch)));
            if cur == 0 {
                break;
            }
            cur -= 1;
        }
    }

    return res;
}

fn main() {
    let mut init = emp_pos();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    init.cors[0][1] = line.chars().nth(3).unwrap();
    init.cors[1][1] = line.chars().nth(5).unwrap();
    init.cors[2][1] = line.chars().nth(7).unwrap();
    init.cors[3][1] = line.chars().nth(9).unwrap();
    line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    init.cors[0][0] = line.chars().nth(3).unwrap();
    init.cors[1][0] = line.chars().nth(5).unwrap();
    init.cors[2][0] = line.chars().nth(7).unwrap();
    init.cors[3][0] = line.chars().nth(9).unwrap();

    let mut visited = HashSet::new();
    let mut cands = BTreeSet::new();
    let mut dists = HashMap::new();
    cands.insert( (0, init.clone()) );
    dists.insert(init.clone(), 0);
    while cands.len() > 0 {
        let elem = cands.iter().next().unwrap().clone();
        if is_final(&elem.1) {
            println!("{}", elem.0);
            return;
        }
        for n in neibs(&elem.1) {
            if visited.contains(&n.0) {
                continue;
            }
            let cost = elem.0 + n.1;
            if dists.contains_key(&n.0) {
                let oldcost = dists.get(&n.0).unwrap();
                if cost < *oldcost {
                    cands.remove(&(*oldcost, n.0.clone()));
                    cands.insert((cost, n.0.clone()));
                    *dists.get_mut(&n.0).unwrap() = cost;
                }
            } else {
                dists.insert(n.0.clone(), cost);
                cands.insert((cost, n.0));
            }
        }
        visited.insert(elem.1.clone());
        cands.remove(&elem);
    }
}
