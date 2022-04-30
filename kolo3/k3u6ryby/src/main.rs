use std::cmp::{max, min};
use std::time::Instant;

fn main() {
    let start_main = Instant::now();
    let mut line = String::new();
    let mut _b1 = std::io::stdin().read_line(&mut line);
    let line_vec = line.trim().split(" ").collect::<Vec<&str>>();
    let swarms: u8 = line_vec[0].parse().unwrap();
    let s: (i32, i32) = (line_vec[1].parse().unwrap(), line_vec[2].parse().unwrap());

    let mut polygons: Vec<Vec<Vec<i32>>> = vec![];
    for _swarm in 0..swarms{
        line = String::new();
        _b1 = std::io::stdin().read_line(&mut line);
        let line_vec2 = line.trim().split(" ").collect::<Vec<&str>>();
        let pol_points: i32 = line_vec2[0].parse().unwrap();
        polygons.push(vec![]);
        for p in 0..pol_points{
            polygons.last_mut().unwrap().push(vec![line_vec2[(1+2*p) as usize].parse().unwrap(), line_vec2[(2+2*p) as usize].parse().unwrap()]);
        }
    }
    let mut cs: Vec<(i32, i32)> = vec![];
    for polygon in polygons{
        let mut max_c: i32 = i32::MIN;
        let mut min_c: i32 = i32::MAX;
        for vertex in polygon{
            let c: i32 = (s.1*vertex[0]) - (s.0*vertex[1]); // c = s_y*x - s_x*y
            max_c = max(max_c, c);
            min_c = min(min_c, c);
        }
        cs.push((max_c, -1));
        cs.push((min_c, 1));
    }
    let mut max_intercepts = 0;
    let mut current_intercepts = 0;
    cs.sort_by_key(|a| (a.0, -a.1));
    for c in cs{
        current_intercepts += c.1;
        max_intercepts = max(max_intercepts, current_intercepts);
        // println!("{:?}", c)
    }
    println!("{}", max_intercepts);
    println!("Main done in: {} μs", start_main.elapsed().as_micros())
}
