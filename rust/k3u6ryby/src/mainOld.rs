use std::cmp::max;

fn magnitude(v: &Vec<f32>) -> f32{
    return v.iter().sum::<f32>().powf(0.5);
}

fn matrix_multiply(a: Vec<Vec<f32>>, b: &Vec<Vec<f32>>) -> Vec<Vec<f32>>{
    let mut out = vec![];
    for (_i, row) in a.iter().enumerate(){
        out.push(vec![]);
        for (ii, _e) in row.iter().enumerate(){
            out.last_mut().unwrap().push(scalar_mul(row, &b.iter().map(|c| c[ii]).collect::<Vec<f32>>()));
        }
    }
    return out;
}

fn scalar_mul(a: &Vec<f32>, b: &Vec<f32>) -> f32{
    let mut out: f32 = 0.0;
    for (i, e) in a.iter().enumerate(){
        out += e*b[i]
    }
    return out;
}

fn intercepts_polygon(start_point: &Vec<f32>, polygon: &Vec<Vec<f32>>) -> bool{
    for (i, p) in polygon.iter().enumerate(){
        let mut i_ = i+1;
        if i == polygon.len()-1{
            i_ = 0;
        }
        if (
            round_decimal(p[0], 6) < round_decimal(start_point[0], 6)
            &&
            round_decimal(polygon[i_][0], 6) < round_decimal(start_point[0], 6)
        )
            ||
        (
            round_decimal(p[0], 6) > round_decimal(start_point[0], 6)
            &&
            round_decimal(polygon[i_][0], 6) > round_decimal(start_point[0], 6)
        ){
            continue;
        }
        else {
            return true;
        }
    }
    return false;
}

fn round_decimal(num: f32, places: u8) -> f32{
    return (num * 10_f32.powf(places.into())).round() / 10_f32.powf(places.into())
}

fn main() {
    let mut line = String::new();
    let mut _b1 = std::io::stdin().read_line(&mut line);
    let line_vec = line.trim().split(" ").collect::<Vec<&str>>();
    let swarms: u8 = line_vec[0].parse().unwrap();
    let s: Vec<f32> = vec![line_vec[1].parse().unwrap(), line_vec[2].parse().unwrap()];
    let sin: f32 = s[0]/magnitude(&s);
    let cos: f32 = -s[1]/magnitude(&s);
    let mut polygons: Vec<Vec<Vec<f32>>> = vec![];
    // let mut pol_points: u8;
    for _swarm in 0..swarms{
        line = String::new();
        _b1 = std::io::stdin().read_line(&mut line);
        let line_vec2 = line.trim().split(" ").collect::<Vec<&str>>();
        let pol_points: u8 = line_vec2[0].parse().unwrap();
        polygons.push(vec![]);
        for p in 0..pol_points{
            polygons.last_mut().unwrap().push(vec![line_vec2[(1+2*p) as usize].parse().unwrap(), line_vec2[(2+2*p) as usize].parse().unwrap()]);
        }
    }
    let mut rotated_polygons = vec![];
    let rot_matrix = vec![vec![cos, -sin], vec![sin, cos]];
    for polygon in polygons{
        rotated_polygons.push(matrix_multiply(polygon, &rot_matrix));
    }
    let mut vertices = vec![];
    for polygon in rotated_polygons.iter(){
        for p in polygon.iter(){
            vertices.push(p);
        }
    }
    // println!("{:?}", rotated_polygons);
    // dbg!(&vertices, vertices.len());
    let mut sorted_vertices_indexes = (0..vertices.len()).collect::<Vec<usize>>();
    sorted_vertices_indexes.sort_by(|a, b| vertices[*a][0].partial_cmp(&vertices[*b][0]).unwrap());
    let mut max_intercepts = 0;
    for vertex_index in sorted_vertices_indexes{
        let mut intercepts = 0;
        for polygon in &rotated_polygons{
            if intercepts_polygon(vertices[vertex_index], polygon){
                intercepts += 1;
            }
        }
        max_intercepts = max(max_intercepts, intercepts);
    }
    println!("{}", max_intercepts)
}
