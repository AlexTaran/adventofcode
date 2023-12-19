use std::io;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct PartRange {
    x0: u32,
    x1: u32,
    m0: u32,
    m1: u32,
    a0: u32,
    a1: u32,
    s0: u32,
    s1: u32,
}

fn volume(pr: &PartRange) -> u64 {
    if pr.x0 > pr.x1 || pr.m0 > pr.m1 || pr.a0 > pr.a1 || pr.s0 > pr.s1 {
        return 0;
    }
    return (pr.x1-pr.x0+1) as u64 * (pr.m1-pr.m0+1) as u64 * (pr.a1-pr.a0+1) as u64 *(pr.s1-pr.s0+1) as u64;
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

fn split(pr: &PartRange, cond: &Cond) -> (Option<PartRange>, Option<PartRange>) {
    let (v0, v1) = match cond.var {
        'x' => (pr.x0, pr.x1),
        'm' => (pr.m0, pr.m1),
        'a' => (pr.a0, pr.a1),
        's' => (pr.s0, pr.s1),
        _ => {
            println!("Bad var");
            (0, 0)
        }
    };
    let mut p = pr.clone();
    let mut f = pr.clone();
    if cond.sign == '<' {
        if v1 < cond.val {
            return (Some(pr.clone()), None);
        } else if cond.val <= v0 {
            return (None, Some(pr.clone()));
        } else {
            // (v0 val v1]
            // v0 .. val-1 ; val .. v1
            match cond.var {
                'x' => {p.x1 = cond.val-1; f.x0 = cond.val;},
                'm' => {p.m1 = cond.val-1; f.m0 = cond.val;},
                'a' => {p.a1 = cond.val-1; f.a0 = cond.val;},
                's' => {p.s1 = cond.val-1; f.s0 = cond.val;},
                _ =>  println!("Bad var")
            }
        }
    } else {
        if v0 > cond.val {
            return (Some(pr.clone()), None);
        } else if cond.val >= v1 {
            return (None, Some(pr.clone()));
        } else {
            // [v0 val v1)
            // val+1 .. v1; v0 .. val
            match cond.var {
                'x' => {p.x0 = cond.val+1; f.x1 = cond.val;},
                'm' => {p.m0 = cond.val+1; f.m1 = cond.val;},
                'a' => {p.a0 = cond.val+1; f.a1 = cond.val;},
                's' => {p.s0 = cond.val+1; f.s1 = cond.val;},
                _ =>  println!("Bad var")
            }
        }
    }
    let sp = if volume(&p) > 0 {Some(p)} else {None};
    let sf = if volume(&f) > 0 {Some(f)} else {None};
    return (sp, sf);
}

fn calc(pr: &PartRange, conds: &HashMap<String, CondList>, cur: &str, rule_idx: usize) -> u64 {
    let cond = &conds.get(cur).unwrap().conds[rule_idx];
    let (pass, fail) = split(pr, cond);
    let mut res:u64 = 0;
    if pass != None {
        if cond.target == "A" {
            res += volume(&pass.unwrap());
        } else if cond.target != "R" {
            res += calc(&pass.unwrap(), conds, &cond.target, 0);
        }
    }
    if fail != None {
        let deftarget = &conds.get(cur).unwrap().deftarget;
        if rule_idx + 1 < conds.get(cur).unwrap().conds.len() {
            res += calc(&fail.unwrap(), conds, cur, rule_idx + 1);
        } else if deftarget == "A" {
            res += volume(&fail.unwrap());
        } else if deftarget != "R" {
            res += calc(&fail.unwrap(), conds, deftarget, 0);
        }
    }
    return res;
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

    let pr = PartRange {
        x0: 1, x1: 4000,
        m0: 1, m1: 4000,
        a0: 1, a1: 4000,
        s0: 1, s1: 4000,
    };
    let cur:String = "in".to_string();
    let res:u64 = calc(&pr, &conds, &cur, 0);
    println!("{}", res);
}
