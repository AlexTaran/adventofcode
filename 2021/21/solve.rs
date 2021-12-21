use std::io;
use std::cmp;

fn main() {
    let mut positions: [i32; 2] = [0, 0];
    for id in 0..2 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim()[28..].to_string();
        positions[id] = line.parse().unwrap();
    }

    let mut scores: [i32; 2] = [0, 0];
    let mut turn = 0;
    let mut nxtdie = 1;
    let mut counter = 0;
    while scores[0] < 1000 && scores[1] < 1000 {
        for _i in 0..3 {
            positions[turn] += nxtdie;
            nxtdie += 1;
            if nxtdie > 100 {
                nxtdie -= 100;
            }
            counter += 1;
        }
        while positions[turn] > 10 {
            positions[turn] -= 10;
        }
        scores[turn] += positions[turn];
        turn = 1 - turn;
    }
    println!("{}", counter * cmp::min(scores[0], scores[1]));
}
