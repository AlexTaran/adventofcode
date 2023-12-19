use std::io;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Clone, Debug)]
struct Cond {
    var: char,
    sign: char,
    val: u32,
    target: String,
}

#[derive(Clone, Debug)]
struct CondList {
    conds: Vec<Cond>,
    deftarget: String,
}

fn apply_condlist(p: &Part, cl: &CondList) -> String {
    for cond in cl.conds.iter() {
        let v = match cond.var {
            'x' => p.x,
            'm' => p.m,
            'a' => p.a,
            's' => p.s,
            _ => {
                println!("Bad var");
                0
            }
        };
        if cond.sign == '<' {
            if v < cond.val {
                return cond.target.to_string();
            }
        } else {
            if v > cond.val {
                return cond.target.to_string();
            }
        }
    }
    return cl.deftarget.to_string();
}

fn main() {
    let mut conds:HashMap<String, CondList> = HashMap::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 || line.trim().len() == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split_terminator(['{', '}']).collect();
        let cparts:Vec<&str> = parts[1].split_terminator([':', ',']).collect();
        let mut cl = CondList {conds: Vec::new(), deftarget: cparts.iter().last().unwrap().to_string()};
        for i in 0..cparts.len() / 2 {
            let args:Vec<&str> = cparts[i*2].split_terminator(['<', '>']).collect();
            let cond = Cond {
                var: args[0].chars().nth(0).unwrap(),
                sign: cparts[i*2].chars().nth(1).unwrap(),
                val: args[1].parse::<u32>().unwrap(),
                target: cparts[i*2+1].to_string() };
            cl.conds.push(cond);
        }
        conds.insert(parts[0].to_string(), cl);
    }

    let mut res:u32 = 0;
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split_terminator(['{', ',', '=', '}']).collect();
        let p = Part{
            x: parts[2].parse::<u32>().unwrap(),
            m: parts[4].parse::<u32>().unwrap(),
            a: parts[6].parse::<u32>().unwrap(),
            s: parts[8].parse::<u32>().unwrap(),
        };
        let mut cur:String = "in".to_string();
        while cur != "A" && cur != "R" {
            cur = apply_condlist(&p, conds.get(&cur).unwrap());
        }
        if cur == "A" {
            res += p.x + p.m + p.a + p.s;
        }
    }
    println!("{}", res);
}
