use std::io;
use std::collections::HashSet;

fn main() {
    let mut seedline = String::new();
    io::stdin().read_line(&mut seedline).expect("");
    let inputnums:Vec<u64> = seedline[seedline.find(':').unwrap()+1..].trim().split(" ").map(|x|x.parse::<u64>().unwrap()).collect();
    let mut ranges:HashSet<(u64, u64)> = HashSet::new();
    for i in 0..inputnums.len() {
        if i % 2 == 0 {
            ranges.insert((inputnums[i], inputnums[i+1]));
        }
    }

    io::stdin().read_line(&mut seedline).expect(""); // skip empty line

    loop {
        let mut header = String::new();
        let num_bytes = io::stdin().read_line(&mut header).expect("");
        if num_bytes == 0 {
            break;
        }
        if header.find(':').is_some() {
            let mut dst:Vec<u64> = Vec::new();
            let mut src:Vec<u64> = Vec::new();
            let mut rng:Vec<u64> = Vec::new();
            loop {
                let mut line = String::new();
                let num_bytes = io::stdin().read_line(&mut line).expect("");
                if num_bytes == 0 || line.trim().len() == 0 {
                    break;
                }
                let parts:Vec<u64> = line.split_whitespace().map(|x|x.parse::<u64>().unwrap()).collect();
                dst.push(parts[0]);
                src.push(parts[1]);
                rng.push(parts[2]);
            }
            let mut mappedranges:HashSet<(u64, u64)> = HashSet::new();
            let mut leftranges:HashSet<(u64, u64)> = HashSet::new();

            for mpid in 0..src.len() {
                for (number, range) in ranges.iter() {
                    if *number + *range < src[mpid] || *number >= src[mpid] + rng[mpid] {
                        leftranges.insert((*number, *range));
                    } else if *number >= src[mpid] && *number + *range <= src[mpid] + rng[mpid] {
                        mappedranges.insert((*number - src[mpid] + dst[mpid], *range));
                    } else {
                        if *number < src[mpid] {
                            leftranges.insert((*number, src[mpid] - *number));
                        }
                        if *number + *range > src[mpid] + rng[mpid] {
                            leftranges.insert((src[mpid]+rng[mpid], *number+*range-src[mpid]-rng[mpid]));
                        }
                        let innerleft  = number.max(&src[mpid]);
                        let innerright = (*number + *range).min(src[mpid]+rng[mpid]);
                        mappedranges.insert((innerleft-src[mpid]+dst[mpid], innerright - innerleft));
                    }
                }
                ranges = leftranges;
                leftranges = HashSet::new();
            }
            ranges = ranges.union(&mappedranges).cloned().collect();
        }
    }
    println!("{}", ranges.iter().map(|pair|pair.0).min().unwrap());
}
