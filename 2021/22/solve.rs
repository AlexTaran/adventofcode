use std::io;
use std::cmp;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq)]
struct V3 {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut field: HashSet<V3> = HashSet::new(); 
	loop {                                                             
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.len() == 0 {
            break;
        }
        let spl:Vec<&str> = line.trim().split(" ").collect();
        let comps:Vec<&str> = spl[1].split(",").collect();
        let mut coords = Vec::new();
        for i in 0..3 {
            let crd:Vec<&str> = comps[i].split("=").collect();
            let mnmx:Vec<i32> = crd[1].split("..").map(|x|x.parse::<i32>().unwrap()).collect();
            coords.push(mnmx[0]);
            coords.push(mnmx[1]);
        }
        let mn = V3 {x: coords[0], y: coords[2], z: coords[4]};
        let mx = V3 {x: coords[1], y: coords[3], z: coords[5]};

        for x in cmp::max(mn.x, -50) .. cmp::min(mx.x + 1, 51) {
            for y in cmp::max(mn.y, -50) .. cmp::min(mx.y + 1, 51) {
                for z in cmp::max(mn.z, -50) .. cmp::min(mx.z + 1, 51) {
                    let cur = V3 {x: x, y: y, z: z};
                    if spl[0] == "on" {
                        field.insert(cur);
                    } else {
                        field.remove(&cur);
                    }
                }
            }
        }
	}

    println!("{}", field.len());
}
