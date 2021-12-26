use std::cmp::{max};
use std::time::Instant;

fn main() {
    let start_main = Instant::now();
    let mut line = String::from("");
    let mut _b1 = std::io::stdin().read_line(&mut line);
    let num_of_problems: u8 = line.replace("\r\n", "").replace("\n", "").parse().unwrap();

    // println!("{}", num_of_problems);
    for problem_n in 0..num_of_problems{
        println!("Problem {}/{}", problem_n+1, num_of_problems);
        let start = Instant::now();
        let mut line = String::new();
        _b1 = std::io::stdin().read_line(&mut line);
        let input = line.split(" ").collect::<Vec<&str>>();
        let w: i64 = input[0].parse().unwrap();
        let h: i64 = input[1].parse().unwrap();
        let x: i64 = input[2].parse().unwrap();
        let y: i64 = input[3].parse().unwrap();
        let t: i64 = input[4].replace("\r\n", "").replace("\n", "").parse().unwrap();

        let mut t_0 = t;
        if t > w+h{
            t_0 = w+h;
            if (t % 2 == 0 && t_0 % 2 == 1) && (t % 2 == 1 && t_0 % 2 == 0){
                t_0 += 1
            }
        }
        let mut positions: i64 = (t_0+1)*(t_0+1);
        let mut a = max(0, 2*(t_0-(h-y))+1);
        let mut b = max(0, 2*(t_0-(w-x))+1);
        let mut c = max(0, 2*(t_0-(x+1))+1);
        let mut d = max(0, 2*(t_0-(y+1))+1);
        while a > 0 || b > 0 || c > 0 || d > 0{
            positions -= a + b + c + d;
            positions += max(0, t_0-(h-y+w-x)+1);
            positions += max(0, t_0-(w-x+y+1)+1);
            positions += max(0, t_0-(h-y+x+1)+1);
            positions += max(0, t_0-(y+1+x+1)+1);
            t_0 -= 2;
            a = max(0, 2*(t_0-(h-y))+1);
            b = max(0, 2*(t_0-(w-x))+1);
            c = max(0, 2*(t_0-(x+1))+1);
            d = max(0, 2*(t_0-(y+1))+1);
            // println!("Intermediate positions {}", positions)
        }
        println!("{}", positions);
        println!("Problem done in: {} μs", start.elapsed().as_micros())
    }
    println!("Main done in: {} μs", start_main.elapsed().as_micros())
}
