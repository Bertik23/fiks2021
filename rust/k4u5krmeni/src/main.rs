use std::collections::HashMap;

fn main() {
    let mut line = String::new();
    let mut _b1 = std::io::stdin().read_line(&mut line);
    let line_vec = line.split(" ").map(|a| a.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let num_of_countries = line_vec[0];
    let num_of_days = line_vec[1];
    println!("{}, {}", num_of_countries, num_of_days);
    let mut road_graph = HashMap::new();
    for _ in 0..num_of_countries{
        line.clear();
        _b1 = std::io::stdin().read_line(&mut line);
        let line_vec = line.split(" ").map(|a| a.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
    }
}
