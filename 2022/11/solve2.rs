use std::io;

struct Monkey {
    items: Vec<u64>,
    op: char,
    arg: String,
    divby: u64,
    iftrue: i32,
    iffalse: i32,
}

fn inspect(wlevel: u64, monkey: &Monkey, modulo: u64) -> u64 {
    if monkey.arg == "old" {
        if monkey.op == '*' {
            return (wlevel * wlevel) % modulo;
        }
    } else {
        let arg = monkey.arg.parse::<u64>().unwrap();
        if monkey.op == '*' {
            return (wlevel * arg) % modulo;
        } else if monkey.op == '+' {
            return (wlevel + arg) % modulo;
        }
    }
    println!("Bad op");
    return 0;
}

fn main() {
    let mut monkeys = Vec::new();
    let mut modulo = 1;
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        if line.trim().len() == 0 {
            continue;
        }
        let parts:Vec<&str> = line.trim().split(" ").collect();
        if parts[0] != "Monkey" {
            println!("Bad input {}", parts[0]);
            return;
        }

        line = String::new();
        io::stdin().read_line(&mut line).expect("");
        let items:Vec<u64> = line[17..].trim().split(", ").map(|x|x.parse::<u64>().unwrap()).collect();

        line = String::new();
        io::stdin().read_line(&mut line).expect("");
        let opparts:Vec<&str> = line.trim().split(" ").collect();
        let op:char= opparts[4].chars().nth(0).unwrap();
        let arg: String = opparts[5].to_string();
        
        line = String::new();
        io::stdin().read_line(&mut line).expect("");
        let dbyparts:Vec<&str> = line.trim().split(" ").collect();
        let divby: u64 = dbyparts.last().unwrap().parse::<u64>().unwrap();
        modulo *= divby;
        
        line = String::new();
        io::stdin().read_line(&mut line).expect("");
        let iftparts:Vec<&str> = line.trim().split(" ").collect();
        let iftrue: i32 = iftparts.last().unwrap().parse::<i32>().unwrap();
        
        line = String::new();
        io::stdin().read_line(&mut line).expect("");
        let iffparts:Vec<&str> = line.trim().split(" ").collect();
        let iffalse: i32 = iffparts.last().unwrap().parse::<i32>().unwrap();

        monkeys.push(Monkey {
            items: items,
            op: op,
            arg: arg,
            divby: divby,
            iftrue: iftrue,
            iffalse: iffalse,
        });
    }
    let mut counters = vec![0; monkeys.len()];
    for _round in 0..10000 {
        for turn in 0..monkeys.len() {
            let items = monkeys[turn].items.clone();
            counters[turn] += items.len();
            for item in items {
                let wlevel = inspect(item, &monkeys[turn], modulo);
                if wlevel % monkeys[turn].divby == 0 {
                    let idx = monkeys[turn].iftrue as usize;
                    monkeys[idx].items.push(wlevel);
                } else {
                    let idx = monkeys[turn].iffalse as usize;
                    monkeys[idx].items.push(wlevel);
                }
            }
            monkeys[turn].items = Vec::new();
        }
    }
    counters.sort();
    counters.reverse();
    println!("{}", counters[0]*counters[1]);
}
