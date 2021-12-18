use std::io;

struct Elem {
    value: Option<u32>,
    ch: Option<char>,
}

fn clone_elem(e: &Elem) -> Elem {
    return Elem {value: e.value, ch: e.ch};
}

fn to_elems(s: &str) -> Vec<Elem> {
    let mut res = Vec::new();
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            res.push(Elem {value: Some(ch.to_string().parse::<u32>().unwrap()), ch: None} );
        } else {
            res.push(Elem {value: None, ch: Some(ch)});
        }
    }
    return res;
}

fn add(left: &Vec<Elem>, right: &Vec<Elem>) -> Vec<Elem> {
    let mut res = Vec::new();
    res.push(Elem {value: None, ch: Some('[')});
    for e in left.iter() {
        res.push(clone_elem(e));
    }
    res.push(Elem {value: None, ch: Some(',')});
    for e in right.iter() {
        res.push(clone_elem(e));
    }
    res.push(Elem {value: None, ch: Some(']')});
    return res;
}

fn explode(num: &Vec<Elem>) -> (Vec<Elem>, bool) {
    let mut depth = 0;
    let mut found: Option<usize> = None;
    for i in 0..num.len() {
        if num[i].ch == Some('[') {
            depth += 1;
        }
        if num[i].ch == Some(']') {
            depth -= 1;
        }
        if depth >= 5 {
            found = Some(i);
            break;
        }
    }
    let mut res = Vec::new();
    if found == None {
        return (res, false);
    }
    let lft = found.unwrap() + 1;
    let rgh = lft + 2;
    for i in 0..(lft-1) {
        res.push(clone_elem(&num[i]));
    }
    for i in (0..res.len()).rev() {
        if !res[i].value.is_none() {
            let val = res[i].value.as_ref().unwrap() + num[lft].value.as_ref().unwrap();
            res[i] = Elem{value: Some(val), ch: None};
            break;
        }
    }
    let mut pushed_right = false;
    res.push(Elem{value: Some(0), ch: None});
    for i in (rgh+2)..num.len() {
        if !pushed_right && !num[i].value.is_none() {
            pushed_right = true;
            let val = num[i].value.as_ref().unwrap() + num[rgh].value.as_ref().unwrap();
            res.push(Elem{value: Some(val), ch: None});
        } else {
            res.push(clone_elem(&num[i]));
        }
    }
    return (res, true);
}

fn split(num: &Vec<Elem>) -> (Vec<Elem>, bool) {
    let mut found: Option<usize> = None;
    for i in 0..num.len() {
        if !num[i].value.is_none() && *num[i].value.as_ref().unwrap() >= 10 {
            found = Some(i);
            break;
        }
    }
    let mut res = Vec::new();
    if found == None {
        return (res, false);
    }
    let pos = found.unwrap();
    for i in 0..pos {
        res.push(clone_elem(&num[i]));
    }
    let val:u32 = *num[pos].value.as_ref().unwrap();
    res.push(Elem{value: None, ch: Some('[')});
    res.push(Elem{value: Some(val/2), ch: None});
    res.push(Elem{value: None, ch: Some(',')});
    res.push(Elem{value: Some(val - val/2), ch: None});
    res.push(Elem{value: None, ch: Some(']')});
    for i in (pos+1)..num.len() {
        res.push(clone_elem(&num[i]));
    }
    return (res, true);
}

fn reduce(num: &Vec<Elem>) -> Vec<Elem> {
    let mut cur = Vec::new();
    for e in num.iter() {
        cur.push(clone_elem(e));
    }
    loop {
        let (res, f) = explode(&cur);
        if f {
            cur = res;
            continue;
        }
        let (res, f) = split(&cur);
        if f {
            cur = res;
            continue;
        }
        break;
    }
    return cur;
}

fn reduce_magnitude(num: &Vec<Elem>) -> Vec<Elem> {
    let mut res:Vec<Elem> = Vec::new();
    let mut i = 0;
    while i < num.len() {
        if num[i].ch == Some('[') && i + 4 < num.len() {
            if !num[i+1].value.is_none() && num[i+2].ch == Some(',') && !num[i+3].value.is_none() && num[i+4].ch == Some(']') {
                let l = *num[i+1].value.as_ref().unwrap();
                let r = *num[i+3].value.as_ref().unwrap();
                res.push(Elem{value: Some(l*3+r*2), ch:None});
                i += 5;
            } else {
                res.push(clone_elem(&num[i]));
                i += 1;
            }
        } else {
            res.push(clone_elem(&num[i]));
            i += 1;
        }
    }
    return res;
}

fn magnitude(num: &Vec<Elem>) -> u32 {
    let mut cur = Vec::new();
    for e in num.iter() {
        cur.push(clone_elem(e));
    }
    loop {
        if cur.len() == 1 {
            return cur[0].value.unwrap();
        }
        cur = reduce_magnitude(&cur);
    }
}

fn main() {
    let mut nums = Vec::new();
	loop {                                                             
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().len() == 0 {
            break;
        }
        nums.push(to_elems(&line.trim()));
	}
    let mut max = 0;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            let res = add(&nums[i], &nums[j]);
            let res = reduce(&res);
            let m = magnitude(&res);
            if m > max {
                max = m;
            }
        }
    }
    println!("{}", max);
}
