use std::io;
use std::collections::HashMap;
use std::collections::HashSet;

fn find_paths(g: &HashMap<String, Vec<String>>, v: String, visited: HashSet<String>, twice: bool) -> u32 {
    if v == "end" {
        return 1;
    }
    let mut counter: u32 = 0;
    let neibs = g.get(&v).unwrap();
    let mut newvis = visited.clone();
    let mut newtwice = twice;
    if v.to_lowercase() == v {
        if newvis.contains(&v) {
            newtwice = true;
        } else {
            newvis.insert(v.to_string());
        }
    }
    for neib in neibs {
        if neib == "start" {
            continue;
        }
        if neib.to_lowercase() == neib.to_string() && (newvis.contains(neib) && newtwice) {
            continue;
        }
        counter += find_paths(g, neib.to_string(), newvis.clone(), newtwice);
    }
    return counter;
}

fn main() {
    let mut g = HashMap::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().len() == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split("-").collect();
        let p1 = String::from(parts[0]);
        let p2 = String::from(parts[1]);
        if !g.contains_key(&p1) {
            g.insert(p1.clone(), Vec::new());
        }
        if !g.contains_key(&p2) {
            g.insert(p2.clone(), Vec::new());
        }
        g.get_mut(&p1).unwrap().push(p2.clone());
        g.get_mut(&p2).unwrap().push(p1.clone());
    }
    println!("{}", find_paths(&g, "start".to_string(), HashSet::new(), false));
}
