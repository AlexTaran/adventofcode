use std::io;

fn main() {
    let mut sm:u32 = 0;
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin().read_line(&mut line).expect("");
        if num_bytes == 0 {
            break;
        }
        if line.trim().len() == 0 {
            break;
        }
        let l = line.trim().len();
        let a = &line.trim()[0..l/2];
        let b = &line.trim()[l/2..l];
        for i in 0..l/2 {
          let ch = a.chars().nth(i).unwrap();
          let mut cnt = 0;
          for j in 0..l/2 {
              if b.chars().nth(j).unwrap() == ch {
                  cnt+=1;
              }
          }
          if cnt == 0 {
              continue;
          }
          let code = ch as u32;
          if code >= 'a' as u32 && code <= 'z' as u32 {
            sm += code - 'a' as u32 + 1;
          } else {
            sm += code - 'A' as u32 + 27;
          }
          //println!("{}", ch);

          break;
        }
        //println!("{} {}", a, b);
    }
    println!("{}", sm);
}
