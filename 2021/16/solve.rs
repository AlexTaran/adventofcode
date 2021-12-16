use std::io;

struct Packet {
    version: u32,
    children: Vec<Packet>,
}

fn parse (s: &str) -> (Packet, usize) {
    let mut cur = 0;

    let version = u32::from_str_radix(&s[cur..(cur+3)], 2).unwrap();
    let type_id = u32::from_str_radix(&s[(cur+3)..(cur+6)], 2).unwrap();
    cur += 6;
    if type_id == 4 { // literal
        let mut valstr = String::new();
        loop {
            let sub = &s[(cur+1)..(cur+5)];
            let flag = s.chars().nth(cur).unwrap();
            cur += 5;
            valstr.push_str(sub);
            if  flag == '0' {
                break;
            }
        }
        let _value = u64::from_str_radix(&valstr, 2).unwrap();
        return (Packet {
            version: version,
            children: Vec::new()
        }, cur);
    }
    let length_type_id = u8::from_str_radix(&s[cur..(cur+1)], 2).unwrap();
    cur += 1;
    let mut length_value = 0;
    match length_type_id {
        0 => {length_value = u32::from_str_radix(&s[cur..(cur+15)], 2).unwrap(); cur += 15;},
        1 => {length_value = u32::from_str_radix(&s[cur..(cur+11)], 2).unwrap(); cur += 11;},
        _ => println!("error")
    }
    let mut children:Vec<Packet> = Vec::new();
    if length_type_id == 1 {
        for _k in 0..length_value {
            let (packet, sz) = parse(&s[cur..]);
            children.push(packet);
            cur += sz;
        }
    } else {
        let mut total:usize = 0;
        while total < length_value as usize {
            let (packet, sz) = parse(&s[cur..]);
            children.push(packet);
            total += sz;
            cur += sz;
        }
    }
    return (Packet {
        version: version,
        children: children
    }, cur);
}

fn sum_versions(p: &Packet) -> u32 {
    let mut res = p.version;
    for pp in p.children.iter() {
        res += sum_versions(&pp);
    }
    return res;
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line = line.trim().to_string();
    let mut bits = String::new();
    for ch in line.chars() {
        match ch {
            '0' => bits.push_str("0000"),
            '1' => bits.push_str("0001"),
            '2' => bits.push_str("0010"),
            '3' => bits.push_str("0011"),
            '4' => bits.push_str("0100"),
            '5' => bits.push_str("0101"),
            '6' => bits.push_str("0110"),
            '7' => bits.push_str("0111"),
            '8' => bits.push_str("1000"),
            '9' => bits.push_str("1001"),
            'A' => bits.push_str("1010"),
            'B' => bits.push_str("1011"),
            'C' => bits.push_str("1100"),
            'D' => bits.push_str("1101"),
            'E' => bits.push_str("1110"),
            'F' => bits.push_str("1111"),
            _ => println!("error")
        }
    }
    let (root, _sz) = parse(&bits);
    
    println!("{}", sum_versions(&root));
}
