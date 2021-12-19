use std::io;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone)]
struct V3 {
    x: i32,
    y: i32,
    z: i32,
}

// Column-major
struct M3 {
    data: [i32; 9]
}

fn id_m() -> M3 {
    return M3 {data: [
        1, 0, 0,
        0, 1, 0,
        0, 0, 1,
    ]};
}

fn rotx_m() -> M3 {
    return M3 {data: [
        1, 0, 0,
        0, 0,-1,
        0, 1, 0,
    ]};
}

fn roty_m() -> M3 {
    return M3 {data: [
        0, 0,-1,
        0, 1, 0,
        1, 0, 0,
    ]};
}

fn rotz_m() -> M3 {
    return M3 {data: [
        0,-1, 0,
        1, 0, 0,
        0, 0, 1,
    ]};
}

fn mul_m(a: &M3, b: &M3) -> M3 {
    return M3 {data: [
        a.data[0] * b.data[0] + a.data[3] * b.data[1] + a.data[6] * b.data[2],
        a.data[1] * b.data[0] + a.data[4] * b.data[1] + a.data[7] * b.data[2],
        a.data[2] * b.data[0] + a.data[5] * b.data[1] + a.data[8] * b.data[2],

        a.data[0] * b.data[3] + a.data[3] * b.data[4] + a.data[6] * b.data[5],
        a.data[1] * b.data[3] + a.data[4] * b.data[4] + a.data[7] * b.data[5],
        a.data[2] * b.data[3] + a.data[5] * b.data[4] + a.data[8] * b.data[5],

        a.data[0] * b.data[6] + a.data[3] * b.data[7] + a.data[6] * b.data[8],
        a.data[1] * b.data[6] + a.data[4] * b.data[7] + a.data[7] * b.data[8],
        a.data[2] * b.data[6] + a.data[5] * b.data[7] + a.data[8] * b.data[8],
    ]};
}

fn eq_m(a: &M3, b: &M3) -> bool {
    for i in 0..9 {
        if a.data[i] != b.data[i] {
            return false;
        }
    }
    return true;
}

fn all_rots() -> Vec<M3> {
    let mut res = Vec::new();
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                let mut m = id_m();
                for _ii in 0..i {
                    m = mul_m(&m, &rotx_m());
                }
                for _jj in 0..j {
                    m = mul_m(&m, &roty_m());
                }
                for _kk in 0..k {
                    m = mul_m(&m, &rotz_m());
                }
                let mut found = false;
                for fm in res.iter() {
                    if eq_m(&m, fm) {
                        found = true;
                        break;
                    }
                }
                if !found {
                    res.push(m);
                }
            }
        }
    }
    return res;
}

fn add(a: &V3, b: &V3) -> V3 {
    return V3 {x: a.x+b.x, y: a.y+b.y, z: a.z+b.z};
}

fn sub(a: &V3, b: &V3) -> V3 {
    return V3 {x: a.x-b.x, y: a.y-b.y, z: a.z-b.z};
}

fn mul(m: &M3, v: &V3) -> V3 {
    return V3 {
        x: m.data[0] * v.x + m.data[3] * v.y + m.data[6] * v.z,
        y: m.data[1] * v.x + m.data[4] * v.y + m.data[7] * v.z,
        z: m.data[2] * v.x + m.data[5] * v.y + m.data[8] * v.z,
    };
}

fn transform_beacons(data: &HashSet<V3>, m: &M3) -> HashSet<V3> {
    let mut res = HashSet::new();
    for v in data.iter() {
        res.insert(mul(m, v));
    }
    return res;
}

fn clone(v: &V3) -> V3 {
    return V3 {x: v.x, y: v.y, z: v.z};
}

fn try_merge(data1: &HashSet<V3>, data2: &HashSet<V3>) -> (HashSet<V3>, V3) {
    for vi in data1.iter() {
        for vj in data2.iter() {
            let trvec = sub(vi, vj);
            let mut counter = 1;
            for vii in data1.iter() {
                if vii == vi {
                    continue;
                }
                if data2.contains(&sub(vii, &trvec)) {
                    counter += 1;
                }
            }
            if counter >= 12 {
                let mut res = HashSet::new();
                for vii in data1.iter() {
                    res.insert(clone(vii));
                }
                for vjj in data2.iter() {
                    if !data1.contains(&add(&trvec, vjj)) {
                        res.insert(add(&trvec, vjj));
                    }
                }
                return (res, trvec);
            }
        }
    }
    return (HashSet::new(), V3 {x: 0, y: 0, z: 0}); // empty means failure
}

fn main() {
    let mut inp: Vec<HashSet<V3>> = Vec::new(); 
    let mut beacs: Vec<Vec<V3>> = Vec::new();
	loop {                                                             
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.len() == 0 {
            break;
        }
        let mut data = HashSet::new();
        loop {
            let mut vecline = String::new();
            io::stdin().read_line(&mut vecline).unwrap();
            if vecline.trim().len() == 0 {
                break;
            }
            let nums:Vec<i32> = vecline.trim().split(",").map(|x|x.parse::<i32>().unwrap()).collect();
            data.insert(V3 {x: nums[0], y: nums[1], z: nums[2]} );
        }
        inp.push(data);
        beacs.push(vec![V3 {x: 0, y: 0, z: 0}]);
	}
    let rots = all_rots();
    while inp.len() > 1 {
        let mut newinp = Vec::new();
        let mut newbeacs = Vec::new();
        let mut merged = false;
        for i in 0..inp.len() {
            for j in (i+1)..inp.len() {
                for rot in rots.iter() {
                    let data = transform_beacons(&inp[j], rot);
                    let (mrg, delta) = try_merge(&inp[i], &data);
                    if mrg.len() > 0 {
                        for k in 0..inp.len() {
                            if k == i || k == j {
                                continue;
                            }
                            newinp.push(transform_beacons(&inp[k], &id_m()));
                            newbeacs.push(beacs[k].clone());
                        }
                        newinp.push(mrg);
                        let mut mrgbeacs = beacs[i].clone();
                        for vb in beacs[j].iter() {
                            mrgbeacs.push(add(&mul(rot, vb), &delta))
                        }
                        newbeacs.push(mrgbeacs);
                        merged = true;
                        break;
                    }
                }
                if merged {
                    break;
                }
            }
            if merged {
                break;
            }
        }
        inp = newinp;
        beacs = newbeacs;
    }

    let mut mxdist = 0;
    for i in 0..beacs[0].len() {
        for j in (i+1)..beacs[0].len() {
            let vd = sub(&beacs[0][i], &beacs[0][j]);
            let d = vd.x.abs() + vd.y.abs() + vd.z.abs();
            if mxdist < d {
                mxdist = d;
            }
        }
    }

    println!("{}", mxdist);
}
