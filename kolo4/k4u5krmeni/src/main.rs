use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn get_distances(current_node: u32, graph: &HashMap<u32, Vec<u32>>, visited_nodes: Vec<u32>, distances: &mut HashMap<u32,HashMap<u32,u32>>){
    //println!("{:?}", visited_nodes);
    let mut visited_nodes_upd = visited_nodes.clone();
    let current_distance = visited_nodes.len();
    for (i, node) in visited_nodes.into_iter().enumerate(){
        distances.get_mut(&node).unwrap().insert(current_node, (current_distance - i) as u32);
    }
    visited_nodes_upd.push(current_node);
    //dbg!(&current_node);
    if graph.contains_key(&current_node){
        for node in graph[&current_node].clone(){
            get_distances(node, graph, visited_nodes_upd.clone(), distances);
        }
    }
}

fn find_distance(start: u32, end: u32, graph: &HashMap<u32, Vec<u32>>, inverse_graph: &HashMap<u32, u32>, distances: &mut HashMap<u32, HashMap<u32,u32>>) -> u32{
    if start == end{
        return 0u32
    }
    let d = distances.get(&start);
    if d == None{
        let distance = find_distance(inverse_graph[&start], end, graph, inverse_graph, distances) + 1;
        distances.insert(start, HashMap::from([(end, distance)]));
    } else {
        if d.unwrap().contains_key(&end){
            //dbg!(&distances[&2]);
            return d.unwrap()[&end]
        } else {
            let distance = find_distance(inverse_graph[&start], end, graph, inverse_graph, distances) + 1;
            //dbg!(&distances[&2], start, distance);
            distances.get_mut(&start).unwrap().insert(end, distance);
            //dbg!(&distances[&2]);
        }
    }
    //dbg!(&distances[&2]);
    return distances[&start][&end]
}

fn get_optimal_meeting_point(points: Vec<u32>, graph: &HashMap<u32, Vec<u32>>, inverse_graph: &HashMap<u32, u32>, distances: &mut HashMap<u32, HashMap<u32,u32>>) -> u32{
    let mut current_point = 1u32;
    let mut min_distance: u32 = points.iter().map(|x| find_distance(current_point, *x, graph, inverse_graph, distances)).sum();
    let mut neighbor_distances: HashMap<u32, u32> = HashMap::new();
    loop {
        //dbg!(&distances[&2]);
        neighbor_distances.clear();
        for node in graph.get(&current_point).unwrap_or(&vec![]){
            //dbg!(points.iter().map(|x| find_distance(*node, *x, graph, inverse_graph, distances)).collect::<Vec<u32>>());
            neighbor_distances.insert(*node, points.iter().map(|x| find_distance(*node, *x, graph, inverse_graph, distances)).sum());
        }
        if current_point != 1{
            neighbor_distances.insert(inverse_graph[&current_point], points.iter().map(|x| find_distance(inverse_graph[&current_point], *x, graph, inverse_graph, distances)).sum());
        }
        let min_neighbor_distance = neighbor_distances.iter().min_by_key(|x| x.1).unwrap();
        //dbg!(&neighbor_distances);
        //dbg!(&min_neighbor_distance);
        if min_neighbor_distance.1 >= &min_distance{
            return current_point
        } else{
            current_point = *min_neighbor_distance.0;
            min_distance = *min_neighbor_distance.1;
        }
    }
}

fn main() {
    let start_main = Instant::now();
    let mut line = String::new();
    let mut _b1 = std::io::stdin().read_line(&mut line);
    let line_vec = line.split(" ").map(|a| a.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let num_of_countries = line_vec[0];
    let num_of_days = line_vec[1];
    // println!("{}, {}", num_of_countries, num_of_days);
    let mut prep_road_graph: HashMap<u32,Vec<u32>> = HashMap::new();
    let mut road_graph: HashMap<u32,Vec<u32>> = HashMap::new();
    let mut inverse_road_graph: HashMap<u32,u32> = HashMap::new();
    let mut road_distances: HashMap<u32,HashMap<u32,u32>> = HashMap::new();
    let start_prep = Instant::now();
    for _ in 0..num_of_countries-1{
        line.clear();
        _b1 = std::io::stdin().read_line(&mut line);
        let line_vec = line.split(" ").map(|a| a.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
        // road_graph.get_mut(&line_vec[0]).unwrap_or(&mut vec![]).push(line_vec[1]);
        if prep_road_graph.contains_key(&line_vec[0]){
            prep_road_graph.get_mut(&line_vec[0]).unwrap().push(line_vec[1]);
            // inverse_road_graph.insert(line_vec[1], line_vec[0]);
            road_distances.get_mut(&line_vec[0]).unwrap().insert(line_vec[1], 1u32);
        } else {
            prep_road_graph.insert(line_vec[0], vec![line_vec[1]]);
            // inverse_road_graph.insert(line_vec[1], line_vec[0]);
            road_distances.insert(line_vec[0], HashMap::from([(line_vec[1], 1u32)]));
        }
        if prep_road_graph.contains_key(&line_vec[1]){
            prep_road_graph.get_mut(&line_vec[1]).unwrap().push(line_vec[0]);
            // inverse_road_graph.insert(line_vec[1], line_vec[0]);
            road_distances.get_mut(&line_vec[1]).unwrap().insert(line_vec[0], 1u32);
        } else {
            prep_road_graph.insert(line_vec[1], vec![line_vec[0]]);
            // inverse_road_graph.insert(line_vec[1], line_vec[0]);
            road_distances.insert(line_vec[1], HashMap::from([(line_vec[0], 1u32)]));
        }
    }
    
    let mut visited_nodes: HashSet<u32> = HashSet::new();

    let mut queue = vec![1u32];
    for current_node_index in 0..num_of_countries as usize{
        visited_nodes.insert(queue[current_node_index]);
        for n in &prep_road_graph[&queue[current_node_index]]{
            if !visited_nodes.contains(&n){
                queue.push(*n);
                if road_graph.contains_key(&queue[current_node_index]){
                    road_graph.get_mut(&queue[current_node_index]).unwrap().push(*n);
                    inverse_road_graph.insert(*n, queue[current_node_index]);
                } else {
                    road_graph.insert(queue[current_node_index], vec![*n]);
                    inverse_road_graph.insert(*n, queue[current_node_index]);
                }
                // road_graph.get_mut(&queue[current_node_index]).unwrap().pop();
            }
        }
    }

    //dbg!(&road_graph);
    get_distances(1, &road_graph, vec![], &mut road_distances);
    let end_prep = Instant::now();
    let mut query_times = vec![];
    //dbg!(&road_distances);
    //dbg!(&inverse_road_graph);
    for _ in 0..num_of_days{
        let start_query = Instant::now();
        line.clear();
        _b1 = std::io::stdin().read_line(&mut line);
        let line_vec = line.split(" ").map(|a| a.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let cost: u32 = line_vec[1..line_vec.len()].iter().map(|x| road_distances[&1].get(x).unwrap_or(&0)).sum();
        //dbg!(&road_distances[&2]);
        let optimal_meeting_point = get_optimal_meeting_point(line_vec[1..].to_vec(), &road_graph, &inverse_road_graph, &mut road_distances);
        //dbg!(&road_distances[&2]);
        //println!("{} {} {}", cost, optimal_meeting_point, line_vec[1..].iter().map(|x| find_distance(optimal_meeting_point, *x, &road_graph, &inverse_road_graph, &mut road_distances)).sum::<u32>());
        // println!("{}", cost - line_vec[1..].iter().map(|x| find_distance(optimal_meeting_point, *x, &road_graph, &inverse_road_graph, &mut road_distances)).sum::<u32>());
        //dbg!(&road_distances);
        query_times.push((Instant::now() - start_query).as_micros());
    }
    println!("{:?}", query_times);
    println!(
        "Main: {} μs\n\tPrep: {} μs\n\tAvg. query: {} μs",
        (Instant::now()-start_main).as_micros(),
        (end_prep-start_prep).as_micros(),
        query_times.iter().sum::<u128>() / num_of_days as u128
    )
}
