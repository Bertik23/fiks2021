use std::collections::HashMap;

fn get_corner(wall_size: i32, x: i32, y: i32) -> String{
    let mut out_string = String::new();
    if y < wall_size/2{
        out_string.push('b');
    } else {
        out_string.push('t');
    }
    if x < wall_size/2{
        out_string.push('l');
    } else {
        out_string.push('r');
    }

    return out_string
}

fn get_distance(from: (i32, i32), to: (i32, i32)) -> f64{
    return ((((from.0-to.0) as i64).pow(2) + ((from.1-to.1) as i64).pow(2)) as f64).sqrt();
}

fn main() {
    let mut line = String::new();
    let mut _b1 = std::io::stdin().read_line(&mut line);
    let line_vec = line.split(" ").map(|a| a.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let wall_size = line_vec[0];
    let num_of_questions = line_vec[1];

    let mut top_left: HashMap<(i32, i32), HashMap<&str, f64>> = HashMap::new();
    let mut bottom_left: HashMap<(i32, i32), HashMap<&str, f64>> = HashMap::new();
    let mut top_right: HashMap<(i32, i32), HashMap<&str, f64>> = HashMap::new();
    let mut bottom_right: HashMap<(i32, i32), HashMap<&str, f64>> = HashMap::new();

    let mut top_left_tl_max = f64::MIN;
    let mut top_left_bl_max = f64::MIN;
    let mut top_left_tr_max = f64::MIN;
    let mut bottom_left_bl_max = f64::MIN;
    let mut bottom_left_tl_max = f64::MIN;
    let mut bottom_left_br_max = f64::MIN;
    let mut top_right_tr_max = f64::MIN;
    let mut top_right_tl_max = f64::MIN;
    let mut top_right_br_max = f64::MIN;
    let mut bottom_right_br_max = f64::MIN;
    let mut bottom_right_tr_max = f64::MIN;
    let mut bottom_right_bl_max = f64::MIN;
    
    for _question in 0..num_of_questions{
        line.clear();
        _b1 = std::io::stdin().read_line(&mut line);
        let line_vec = line[2..].split(" ").map(|a| a.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if line.chars().nth(0).unwrap() == '+'{
            let corner = get_corner(wall_size, line_vec[0], line_vec[1]);
            if corner == "tl"{
                let distance_tl = get_distance((line_vec[0], line_vec[1]), (0, wall_size));
                let mut distance_bl = get_distance((line_vec[0], line_vec[1]), (0, 0));
                let mut distance_tr = get_distance((line_vec[0], line_vec[1]), (wall_size, wall_size));
                if distance_bl > wall_size as f64{
                    distance_bl = distance_tl + wall_size as f64;
                }
                if distance_tr > wall_size as f64{
                    distance_tr = distance_tl + wall_size as f64;
                }
                top_left.insert((line_vec[0], line_vec[1]), HashMap::from([("tl", distance_tl), ("bl", distance_bl), ("tr", distance_tr)]));
                if distance_tl > top_left_tl_max{
                    top_left_tl_max = distance_tl;
                }
                if distance_tr > top_left_tr_max{
                    top_left_tr_max = distance_tr;
                }
                if distance_bl > top_left_bl_max{
                    top_left_bl_max = distance_bl;
                }
            } else if corner == "bl"{
                let distance_bl = get_distance((line_vec[0], line_vec[1]), (0, 0));
                let mut distance_tl = get_distance((line_vec[0], line_vec[1]), (0, wall_size));
                let mut distance_br = get_distance((line_vec[0], line_vec[1]), (wall_size, 0));
                if distance_tl > wall_size as f64{
                    distance_tl = distance_bl + wall_size as f64;
                }
                if distance_br > wall_size as f64{
                    distance_br = distance_bl + wall_size as f64;
                }
                top_left.insert((line_vec[0], line_vec[1]), HashMap::from([("tl", distance_tl), ("bl", distance_bl), ("br", distance_br)]));
                if distance_tl > top_left_tl_max{
                    bottom_left_tl_max = distance_tl;
                }
                if distance_br > top_left_tr_max{
                    bottom_left_br_max = distance_br;
                }
                if distance_bl > top_left_bl_max{
                    bottom_left_bl_max = distance_bl;
                }
            } else if corner == "tr"{
                let distance_tr = get_distance((line_vec[0], line_vec[1]), (wall_size, wall_size));
                let mut distance_tl = get_distance((line_vec[0], line_vec[1]), (0, wall_size));
                let mut distance_br = get_distance((line_vec[0], line_vec[1]), (wall_size, 0));
                if distance_tl > wall_size as f64{
                    distance_tl = distance_tr + wall_size as f64;
                }
                if distance_br > wall_size as f64{
                    distance_br = distance_tr + wall_size as f64;
                }
                top_left.insert((line_vec[0], line_vec[1]), HashMap::from([("tl", distance_tl), ("bl", distance_tr), ("br", distance_br)]));
                if distance_tl > top_left_tl_max{
                    bottom_left_tl_max = distance_tl;
                }
                if distance_br > top_left_tr_max{
                    bottom_left_br_max = distance_br;
                }
                if distance_tr > top_left_bl_max{
                    bottom_left_bl_max = distance_tr;
                }
            } else if corner == "br"{
                let distance_br = get_distance((line_vec[0], line_vec[1]), (wall_size, 0));
                let mut distance_bl = get_distance((line_vec[0], line_vec[1]), (0, 0));
                let mut distance_tr = get_distance((line_vec[0], line_vec[1]), (wall_size, wall_size));
                if distance_bl > wall_size as f64{
                    distance_bl = distance_br + wall_size as f64;
                }
                if distance_tr > wall_size as f64{
                    distance_tr = distance_br + wall_size as f64;
                }
                top_left.insert((line_vec[0], line_vec[1]), HashMap::from([("tl", distance_br), ("bl", distance_bl), ("tr", distance_tr)]));
                if distance_br > top_left_tl_max{
                    top_left_tl_max = distance_br;
                }
                if distance_tr > top_left_tr_max{
                    top_left_tr_max = distance_tr;
                }
                if distance_bl > top_left_bl_max{
                    top_left_bl_max = distance_bl;
                }
            }
        } else if line.chars().nth(0).unwrap() == '-'{
            let corner = get_corner(wall_size, line_vec[0], line_vec[1]);
            if corner == "tl"{
                top_left.remove(&(line_vec[0], line_vec[1]));
                top_left_tl_max = *top_left.values().max_by(|x, y| x.get(&"tl").unwrap().partial_cmp(y.get(&"tl").unwrap()).unwrap()).unwrap_or(&HashMap::from([("tl", f64::MIN)])).get("tl").unwrap();
                top_left_bl_max = *top_left.values().max_by(|x, y| x.get(&"bl").unwrap().partial_cmp(y.get(&"bl").unwrap()).unwrap()).unwrap_or(&HashMap::from([("bl", f64::MIN)])).get("bl").unwrap();
                top_left_tr_max = *top_left.values().max_by(|x, y| x.get(&"tr").unwrap().partial_cmp(y.get(&"tr").unwrap()).unwrap()).unwrap_or(&HashMap::from([("tr", f64::MIN)])).get("tr").unwrap();
            } else if corner == "bl"{
                bottom_left.remove(&(line_vec[0], line_vec[1]));
                bottom_left_bl_max = *bottom_left.values().max_by(|x, y| x.get(&"bl").unwrap().partial_cmp(y.get(&"bl").unwrap()).unwrap()).unwrap_or(&HashMap::from([("bl", f64::MIN)])).get("bl").unwrap();
                bottom_left_tl_max = *bottom_left.values().max_by(|x, y| x.get(&"tl").unwrap().partial_cmp(y.get(&"tl").unwrap()).unwrap()).unwrap_or(&HashMap::from([("tl", f64::MIN)])).get("tl").unwrap();
                bottom_left_br_max = *bottom_left.values().max_by(|x, y| x.get(&"br").unwrap().partial_cmp(y.get(&"br").unwrap()).unwrap()).unwrap_or(&HashMap::from([("br", f64::MIN)])).get("br").unwrap();
            } else if corner == "tr"{
                top_right.remove(&(line_vec[0], line_vec[1]));
                top_right_tr_max = *top_right.values().max_by(|x, y| x.get(&"tr").unwrap().partial_cmp(y.get(&"tr").unwrap()).unwrap()).unwrap_or(&HashMap::from([("tr", f64::MIN)])).get("tr").unwrap();
                top_right_tl_max = *top_right.values().max_by(|x, y| x.get(&"tl").unwrap().partial_cmp(y.get(&"tl").unwrap()).unwrap()).unwrap_or(&HashMap::from([("tl", f64::MIN)])).get("tl").unwrap();
                top_right_br_max = *top_right.values().max_by(|x, y| x.get(&"br").unwrap().partial_cmp(y.get(&"br").unwrap()).unwrap()).unwrap_or(&HashMap::from([("br", f64::MIN)])).get("bl").unwrap();
            } else if corner == "br"{
                bottom_right.remove(&(line_vec[0], line_vec[1]));
                bottom_right_br_max = *bottom_right.values().max_by(|x, y| x.get(&"br").unwrap().partial_cmp(y.get(&"br").unwrap()).unwrap()).unwrap_or(&HashMap::from([("br", f64::MIN)])).get("br").unwrap();
                bottom_right_bl_max = *bottom_right.values().max_by(|x, y| x.get(&"bl").unwrap().partial_cmp(y.get(&"bl").unwrap()).unwrap()).unwrap_or(&HashMap::from([("bl", f64::MIN)])).get("bl").unwrap();
                bottom_right_tr_max = *bottom_right.values().max_by(|x, y| x.get(&"tr").unwrap().partial_cmp(y.get(&"tr").unwrap()).unwrap()).unwrap_or(&HashMap::from([("tr", f64::MIN)])).get("tr").unwrap();
            }
        } else if line.chars().nth(0).unwrap() == '?'{
            let top_left_distance = (line_vec[0] + wall_size - line_vec[1]) as f64;
            let bottom_left_distance = (line_vec[0] + line_vec[1]) as f64;
            let top_right_distance = (wall_size - line_vec[0] + wall_size - line_vec[1]) as f64;
            let bottom_right_distance = (wall_size - line_vec[0] + line_vec[1]) as f64;

            // println!("{:?}", top_left);

            // println!("{:?}",  vec![
            //     top_left_distance + top_left_max,
            //     bottom_left_distance + bottom_left_max,
            //     top_right_distance + top_right_max,
            //     bottom_right_distance + bottom_right_max
            // ]);

            println!("{}", vec![
                top_left_distance + top_left_max,
                bottom_left_distance + bottom_left_max,
                top_right_distance + top_right_max,
                bottom_right_distance + bottom_right_max
            ].iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap().round());
            // print!("\n");
        }
    }
}
