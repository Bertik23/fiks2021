use std::collections::HashMap;

fn main() {
    let mut line = String::new();
    let mut _b1 = std::io::stdin().read_line(&mut line);
    let num_of_problems = line.trim().parse::<u32>().unwrap();
    
    for _problem_n in 0..num_of_problems{
        line.clear();
        _b1 = std::io::stdin().read_line(&mut line);
        let line_vec = line.split(" ").map(|a| a.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
        
        let _num_of_vertices = line_vec[0];
        let num_of_edges = line_vec[1];

        let mut graph: HashMap<u32, u32> = HashMap::new();
        
        for _edge_n in 0..num_of_edges{
            line.clear();
            _b1 = std::io::stdin().read_line(&mut line);
            let line_vec = line.split(" ").map(|a| a.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();

            for vertex in line_vec{
                if graph.contains_key(&vertex){
                    *graph.get_mut(&vertex).unwrap_or(&mut 0u32) += 1;
                } else {
                    graph.insert(vertex, 1);
                }
            }
        }

        let mut odd_degree_vertices = vec![];
        for (vertex, edges) in graph.iter(){
            if edges % 2 == 1{
                odd_degree_vertices.push(vertex);
            }
        }

        if odd_degree_vertices.len() == 0{
            println!("Ano.");
            continue
        } else if odd_degree_vertices.len() % 2 == 1{
            panic!()
        } else{
            println!("Ne.");
            println!("{}", odd_degree_vertices.len() / 2);
            for i in (0..odd_degree_vertices.len()).step_by(2){
                println!("{} {}", odd_degree_vertices[i], odd_degree_vertices[i+1]);
            }
        }
    }
}
