use std::io;
use std::collections::HashSet;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Hash, Eq, PartialEq, Clone, Debug, Ord, PartialOrd)]
struct V3 {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug, Ord, PartialOrd)]
struct Cuboid {
    mn: V3,
    mx: V3,
}

fn volume(c: &Cuboid) -> u64 {
    return (c.mx.x - c.mn.x) as u64 * (c.mx.y - c.mn.y) as u64 * (c.mx.z - c.mn.z) as u64;
}

fn corners(c: &Cuboid) -> Vec<V3> {
    return vec![
        V3 {x: c.mn.x, y: c.mn.y, z: c.mn.z},
        V3 {x: c.mn.x, y: c.mn.y, z: c.mx.z},
        V3 {x: c.mn.x, y: c.mx.y, z: c.mn.z},
        V3 {x: c.mn.x, y: c.mx.y, z: c.mx.z},
        V3 {x: c.mx.x, y: c.mn.y, z: c.mn.z},
        V3 {x: c.mx.x, y: c.mn.y, z: c.mx.z},
        V3 {x: c.mx.x, y: c.mx.y, z: c.mn.z},
        V3 {x: c.mx.x, y: c.mx.y, z: c.mx.z},
    ];
}

fn cuboid_coords(c: &Cuboid) ->Vec<i32> {
    return vec![
        c.mn.x,
        c.mn.y,
        c.mn.z,
        c.mx.x,
        c.mx.y,
        c.mx.z,
    ];
}

// ri is big, rj is small
fn isinside(ri: &Cuboid, rj: &Cuboid) -> bool {
  return ri.mn.x <= rj.mn.x && rj.mx.x <= ri.mx.x && ri.mn.y <= rj.mn.y && rj.mx.y <= ri.mx.y && ri.mn.z <= rj.mn.z && rj.mx.z <= ri.mx.z;
}

fn split(c: &Cuboid, v: &V3) -> Vec<Cuboid> {
    let mut resx = Vec::new();
    if v.x > c.mn.x && v.x < c.mx.x {
        resx.push(Cuboid {mn: V3 {x: c.mn.x, y: c.mn.y, z: c.mn.z}, mx: V3 {x: v.x   , y: c.mx.y, z: c.mx.z}});
        resx.push(Cuboid {mn: V3 {x: v.x   , y: c.mn.y, z: c.mn.z}, mx: V3 {x: c.mx.x, y: c.mx.y, z: c.mx.z}});
    } else {
        resx.push(c.clone());
    }
    let mut resy = Vec::new();
    for b in resx.iter() {
        if v.y > b.mn.y && v.y < b.mx.y {
            resy.push(Cuboid {mn: V3 {x: b.mn.x, y: b.mn.y, z: b.mn.z}, mx: V3 {x: b.mx.x, y: v.y   , z: b.mx.z}});
            resy.push(Cuboid {mn: V3 {x: b.mn.x, y: v.y   , z: b.mn.z}, mx: V3 {x: b.mx.x, y: b.mx.y, z: b.mx.z}});
        } else {
            resy.push(b.clone());
        }
    }
    let mut resz = Vec::new();
    for b in resy.iter() {
        if v.z > b.mn.z && v.z < b.mx.z {
            resz.push(Cuboid {mn: V3 {x: b.mn.x, y: b.mn.y, z: b.mn.z}, mx: V3 {x: b.mx.x, y: b.mx.y, z: v.z   }});
            resz.push(Cuboid {mn: V3 {x: b.mn.x, y: b.mn.y, z: v.z   }, mx: V3 {x: b.mx.x, y: b.mx.y, z: b.mx.z}});
        } else {
            resz.push(b.clone());
        }
    }
    return resz;
}

fn multisplit(c: &Cuboid, pts: &Vec<V3>) -> Vec<Cuboid> {
    let mut res = Vec::new();
    res.push(c.clone());
    for p in pts.iter() {
        let mut newres = Vec::new();
        for r in res.iter() {
            for s in split(r, p) {
                newres.push(s.clone());
            }
        }
        res = newres;
    }
    return res;
}

fn filteroffs(field: &Vec<Cuboid>, offs: &Vec<Cuboid>) -> Vec<Cuboid> {
    let mut res = Vec::new();
    for f in field.iter() {
        let mut exc = false;
        for o in offs.iter() {
            if isinside(o, f) {
                exc = true;
                break;
            }
        }
        if !exc {
            res.push(f.clone());
        }
    }
    return res;
}

fn try_merge(ri: &Cuboid, rj: &Cuboid) -> Option<Cuboid> {
  if ri.mx.x == rj.mn.x && ri.mn.y == rj.mn.y && ri.mx.y == rj.mx.y && ri.mn.z == rj.mn.z && ri.mx.z == rj.mx.z {
    return Some(Cuboid {mn: V3 {x: ri.mn.x, y: ri.mn.y, z: ri.mn.z}, mx: V3 {x: rj.mx.x, y: rj.mx.y, z: rj.mx.z}});
  }
  if ri.mn.x == rj.mn.x && ri.mx.x == rj.mx.x && ri.mx.y == rj.mn.y && ri.mn.z == rj.mn.z && ri.mx.z == rj.mx.z {
    return Some(Cuboid {mn: V3 {x: ri.mn.x, y: ri.mn.y, z: ri.mn.z}, mx: V3 {x: rj.mx.x, y: rj.mx.y, z: rj.mx.z}});
  }
  if ri.mn.x == rj.mn.x && ri.mx.x == rj.mx.x && ri.mn.y == rj.mn.y && ri.mx.y == rj.mx.y && ri.mx.z == rj.mn.z {
    return Some(Cuboid {mn: V3 {x: ri.mn.x, y: ri.mn.y, z: ri.mn.z}, mx: V3 {x: rj.mx.x, y: rj.mx.y, z: rj.mx.z}});
  }
  return None;
}

fn compact(field: &Vec<Cuboid>) -> Vec<Cuboid> {
    let mut rng = thread_rng();
    let mut res = field.clone();
    loop {
        res.shuffle(&mut rng);
        // Creating coord map
        let mut coords:HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..res.len() {
            for crd in cuboid_coords(&res[i]).iter() {
                if !coords.contains_key(crd) {
                    coords.insert(*crd, Vec::new());
                }
                coords.get_mut(crd).unwrap().push(i);
            }
        }
        let mut changed = false;
        for i in 0..res.len() {
            for crd in [res[i].mx.x, res[i].mx.y, res[i].mx.z].iter() {
                let def = Vec::new();
                let cands = coords.get(crd).unwrap_or(&def);
                for cand in cands.iter() {
                    let mrg = try_merge(&res[i], &res[*cand]);
                    if mrg != None {
                        let mut f = Vec::new();
                        for k in 0..res.len() {
                            if k != i && k != *cand {
                                f.push(res[k].clone());
                            }
                        }
                        f.push(mrg.unwrap());
                        res = f;
                        changed = true;
                        break;
                    }
                }
                if changed {
                    break;
                }
            }
            if changed {
                break;
            }
        }
        if !changed {
            break;
        }
    }
    return res;
}

fn turnon(field: &Vec<Cuboid>, c: &Cuboid) -> Vec<Cuboid> {
    let mut sfld = HashSet::new();
    let cc = corners(c);
    for b in field.iter() {
        for sub in multisplit(b, &cc) {
            sfld.insert(sub);
        }
    }
    let mut res = Vec::new();
    for b in sfld.iter() {
        if !isinside(c, b) {
            res.push(b.clone());
        } else {
        }
    }
    res.push(c.clone());
    return res;
}

fn turnoff(field: &Vec<Cuboid>, c: &Cuboid) -> Vec<Cuboid> {
    let mut sfld = HashSet::new();
    let cc = corners(c);
    for b in field.iter() {
        for sub in multisplit(b, &cc) {
            sfld.insert(sub);
        }
    }
    let mut res = Vec::new();
    for b in sfld.iter() {
        if !isinside(c, b) {
            res.push(b.clone());
        } else {
        }
    }
    return res;
}

fn calcvol(f: &Vec<Cuboid>) -> u64 {
    let mut res: u64 = 0;
    for c in f.iter() {
        res += volume(c);
    }
    return res;
}

fn main() {
    let mut ops: Vec<(bool, Cuboid)> = Vec::new();
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
        let mx = V3 {x: coords[1] + 1, y: coords[3] + 1, z: coords[5] + 1};

        if spl[0] == "on" {
            ops.push((true, Cuboid {mn: mn, mx: mx}));
        } else {
            ops.push((false, Cuboid {mn: mn, mx: mx}));
        }
    }

    let mut field: Vec<Cuboid> = Vec::new(); 
    for i in 0..ops.len() {
        field = compact(&field);

        let mut nextoffs = Vec::new();
        for j in (i+1)..ops.len() {
            if ops[j].0 == false {
                nextoffs.push(ops[j].1.clone());
            }
        }
        field = filteroffs(&field, &nextoffs);

        if ops[i].0 {
            field = turnon(&field, &ops[i].1);
        } else {
            field = turnoff(&field, &ops[i].1);
        }
	}

    println!("{}", calcvol(&field));
}
