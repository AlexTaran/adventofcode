use std::io;
use std::collections::HashMap;

fn count(cur:&str, graph: &HashMap<String, Vec<String>>, sizes: &HashMap<String, u32>) -> u32 {
    if sizes.contains_key(cur) {
        return *sizes.get(cur).unwrap();
    }
    let mut cnt = 0;
    if !graph.contains_key(cur) {
        return 0;
    }
    for sub in graph.get(cur).unwrap() {
        cnt += count(sub, graph, sizes);
    }
    return cnt;
}

fn main() {
    let mut parents: HashMap<String, String> = HashMap::new();
    let mut sizes: HashMap<String, u32> = HashMap::new();
    let mut curpath: Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        if line.trim().len() == 0 {
            break;
        }
        let parts:Vec<&str> = line.trim().split(" ").collect();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                if parts[2] == "/" {
                    curpath = Vec::new();
                } else if parts[2] == ".." {
                    curpath = parents.get(&curpath.join("/")).unwrap().split("/").map(|s| s.to_owned()).collect();
                } else {
                    let curdir = curpath.join("/");
                    curpath.push(parts[2].to_string());
                    let inner = curpath.join("/");
                    parents.insert(inner, curdir);
                }
            } else if parts[1] == "ls" {
                continue;
            }
        } else {
            let curdir = curpath.join("/");
            let mut inner = curpath.clone();
            inner.push(parts[1].to_string());
            parents.insert(inner.join("/"), curdir);
            if parts[0] == "dir" {
            } else {
                let sz = parts[0].parse::<u32>().unwrap();
                sizes.insert(inner.join("/"), sz);
            }
        }
    }
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for (cur, par) in parents.iter() {
        if !graph.contains_key(par) {
            graph.insert(par.to_string(), vec![cur.to_string()]);
        } else {
            graph.get_mut(par).unwrap().push(cur.to_string());
        }
    }
    let total = count("", &graph, &sizes);
    let free = 70000000 - total;
    let need = 30000000 - free;
    let mut minfound = 70000000;
    for (d, _subs) in graph.iter() {
       let c = count(d, &graph, &sizes);
       if c >= need && c < minfound {
           minfound = c;
       }
    }
    println!("{}", minfound);
}
