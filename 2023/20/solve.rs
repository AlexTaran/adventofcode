use std::io;
use std::collections::HashMap;
use std::collections::VecDeque;

struct Module {
    t: char,
    ff: bool,
    conj: HashMap<String, bool>,
}

fn create_ff(on: bool) -> Module {
    return Module {t: '%', ff: on, conj: HashMap::new()};
}

fn create_conj() -> Module {
    return Module {t: '&', ff: false, conj: HashMap::new()};
}

struct Pulse {
    high: bool,
    src: String,
    dst: String,
}

fn main() {
    let mut modules:HashMap<String, Module> = HashMap::new();
    let mut initial:Vec<String> = Vec::new();
    let mut graph:HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 || line.trim().len() == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split(" -> ").collect();
        let dests:Vec<&str> = parts[1].split(", ").collect();
        let mut name:String = parts[0].to_string();
        if parts[0] == "broadcaster" {
            for d in dests.iter() {
                initial.push(d.to_string())
            }
        } else if parts[0].chars().nth(0).unwrap() == '%' {
            name = parts[0][1..].to_string();
            modules.insert(name.clone(), create_ff(false) );
        } else if parts[0].chars().nth(0).unwrap() == '&' {
            name = parts[0][1..].to_string();
            modules.insert(name.clone(), create_conj());
        } else {
            println!("Bad module");
        }
        graph.insert(name, dests.iter().map(|&s| s.to_string()).collect());
    }
    for (src, dests) in graph.iter() {
        for dst in dests.iter() {
            if !modules.contains_key(dst) {
                continue;
            }
            if modules.get(dst).unwrap().t == '&' {
                modules.get_mut(dst).unwrap().conj.insert(src.clone(), false);
            }
        }
    }
    let mut lowc:u64 = 0;
    let mut highc:u64 = 0;
    for _n in 0..1000 {
        let mut q = VecDeque::new();
        for d in initial.iter() {
            q.push_back(Pulse{high: false, src: "broadcaster".to_string(), dst: d.clone()});
        }
        lowc += 1; // push button
        while q.len() > 0 {
            let curr = q.pop_front().unwrap();
            if curr.high {
                highc += 1;
            } else {
                lowc += 1;
            }
            if !modules.contains_key(&curr.dst) {
                continue;
            }
            let module = modules.get_mut(&curr.dst).unwrap();
            if module.t == '%' {
                if !curr.high {
                    module.ff = !module.ff;
                    for d in graph.get(&curr.dst).unwrap().iter() {
                        q.push_back(Pulse {high: module.ff, src: curr.dst.clone(), dst: d.clone()});
                    }
                }
            } else {
                module.conj.insert(curr.src.clone(), curr.high);
                let alltrue = module.conj.values().all(|&value| value);
                for d in graph.get(&curr.dst).unwrap().iter() {
                    q.push_back(Pulse {high: !alltrue, src: curr.dst.clone(), dst: d.clone()});
                }
            }
        }
    }
    println!("{}", lowc*highc);
}
