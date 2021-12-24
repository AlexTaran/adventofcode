use std::io;
use std::collections::HashMap;

fn arg2idx(arg: char) -> usize {
    match arg {
        'x' => 0,
        'y' => 1,
        'z' => 2,
        'w' => 3,
        _ => 4
    }
}

fn is_reg(arg: char) -> bool {
    arg == 'x' || arg == 'y' || arg == 'z' || arg == 'w'
}

fn truncate(states: &HashMap<[i64; 4], u64>) -> HashMap<[i64; 4], u64> {
    let mut res = HashMap::new();
    for (s, val) in states.iter() {
        if s[2] > 10000000 {
            continue;
        }
        res.insert(*s, *val);
    }
    return res;
}

fn runcmd(command: &(String, String, String), states: &HashMap<[i64; 4], u64>) -> HashMap<[i64; 4], u64> {
    let mut res = HashMap::new();
    let cmd = &command.0;
    let arg1 = &command.1;
    let arg2 = &command.2;
    for (s, val) in states.iter() {
        let ri = arg2idx(arg1.chars().nth(0).unwrap());
        if cmd == "inp" {
            for digit in 1..10 {
                let mut newreg = s.clone();
                newreg[ri] = digit as i64;
                let newval = val * 10 + digit as u64;
                if res.contains_key(&newreg) {
                    if *res.get(&newreg).unwrap() > newval {
                        *res.get_mut(&newreg).unwrap() = newval;
                    }
                } else {
                    res.insert(newreg, newval);
                }
            }
            continue;
        }
        let mut reg = s.clone();
        let op1 = reg[ri].clone();
        let a2ch0 = arg2.chars().nth(0).unwrap();
        let op2 = if is_reg(a2ch0) { reg[arg2idx(a2ch0)] } else {arg2.parse::<i64>().unwrap()};
        if cmd == "add" {
            reg[ri] = op1 + op2;
        } else if cmd == "mul" {
            reg[ri] = op1 * op2;
        } else if cmd == "div" {
            reg[ri] = op1 / op2;
        } else if cmd == "mod" {
            reg[ri] = op1 % op2;
        } else if cmd == "eql" {
            reg[ri] = if op1 == op2 {1} else {0};
        }
        res.insert(reg, *val);
    }
    return res;
}

fn main() {
    let mut prg = Vec::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.trim().len() == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split(" ").collect();
        let cmd = parts[0].to_string();
        let arg1 = parts[1].to_string();
        let arg2 = if parts.len() == 3 { parts[2].to_string() } else {"".to_string()};
        prg.push( (cmd, arg1, arg2) );
    }
    let mut states = HashMap::new();
    states.insert( [0, 0, 0, 0], 0 );
    for command in prg.iter() {
        states = runcmd(command, &states);
        states = truncate(&states);
    }
    let mut ans = u64::MAX;
    for (s, val) in states.iter() {
        if s[2] == 0 {
            if ans > *val {
                ans = *val;
            }
        }
    }
    println!("{}", ans);
}
