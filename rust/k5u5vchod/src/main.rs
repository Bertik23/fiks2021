use std::collections::HashMap;
// use std::cmp::min_by;

fn get_distance(from: (i32, i32), to: (i32, i32)) -> f64{
    return ((((from.0-to.0) as i64).pow(2) + ((from.1-to.1) as i64).pow(2)) as f64).sqrt();
}

fn get_shortest_distance_to_entrance(house: House, size: i32, distances_from_corners: &HashMap<&str, f64>) -> f64{
    let mut distances_to_corners: HashMap<&str, f64> = HashMap::new();
    if house.x <= 0{
        distances_to_corners.insert("tl", get_distance((house.x, house.y), (0, size)));
        distances_to_corners.insert("bl", get_distance((house.x, house.y), (0, 0)));
        if house.y > size{
            distances_to_corners.insert("tr", get_distance((house.x, house.y), (size, size)));
        } else if house.y < 0{
            distances_to_corners.insert("br", get_distance((house.x, house.y), (size, 0)));
        }
    } else if house.x > 0 && house.x < size{
        if house.y > size{
            distances_to_corners.insert("tl", get_distance((house.x, house.y), (0, size)));
            distances_to_corners.insert("tr", get_distance((house.x, house.y), (size, size)));
        } else if house.y < 0{
            distances_to_corners.insert("bl", get_distance((house.x, house.y), (0, 0)));
            distances_to_corners.insert("br", get_distance((house.x, house.y), (size, 0)));
        }
    } else if house.x >= size{
        distances_to_corners.insert("tr", get_distance((house.x, house.y), (size, size)));
        distances_to_corners.insert("br", get_distance((house.x, house.y), (size, 0)));
        if house.y > size{
            distances_to_corners.insert("tl", get_distance((house.x, house.y), (0, size)));
        } else if house.y < 0{
            distances_to_corners.insert("bl", get_distance((house.x, house.y), (0, 0)));
        }
    }

    // distances_to_corners.iter().min_by_key(|(cor, dist)| distances_from_corners.get(*cor).unwrap() + *dist);
    *distances_to_corners.iter().min_by(
        |(cor0, dist0), (cor1, dist1)|
        (
            distances_from_corners.get(*cor0).unwrap() + *dist0
        ).partial_cmp(
            &(distances_from_corners.get(*cor1).unwrap() + *dist1)).unwrap()
    ).unwrap().1
}

fn get_distances_from_corners(entrance: (i32, i32), size: i32) -> HashMap<&'static str, f64>{
    let mut distances_from_corners: HashMap<&str, f64> = HashMap::new();
    distances_from_corners.insert("tl", (entrance.0 + (size - entrance.1).abs()) as f64);
    distances_from_corners.insert("bl", (entrance.0 + entrance.1) as f64);
    distances_from_corners.insert("br", ((size - entrance.0).abs() + entrance.1) as f64);
    distances_from_corners.insert("tr", ((size - entrance.0).abs() + (size - entrance.1).abs()) as f64);

    return distances_from_corners;
}

#[derive(Clone, Copy)]
struct House{
    x: i32,
    y: i32,
    distance: f64
}

impl House{
    fn new(center: i32, x: i32, y: i32, distance: Option<f64>) -> Self {
        let distance: f64 = match distance {
            Some(len) => len,
            None => get_distance((center, center), (x, y))
        };
        Self {
            x,
            y,
            distance
        }
    }
}

impl PartialEq for House{
    fn eq(&self, other: &Self) -> bool{
        self.distance == other.distance
    }
}

impl Eq for House{}

impl PartialOrd for House{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>{
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for House {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering{
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

fn main() {
    let mut line = String::new();
    let mut _b1 = std::io::stdin().read_line(&mut line);
    let line_vec = line.split(" ").map(|a| a.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let wall_size = line_vec[0];
    let num_of_questions = line_vec[1];

    let mut houses: Vec<House> = vec![];
    let remember_size = 4*wall_size;

    let center = wall_size/2;

    let mut added_house = true;


    for _question in 0..num_of_questions{
        line.clear();
        _b1 = std::io::stdin().read_line(&mut line);
        let line_vec = line[2..].split(" ").map(|a| a.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if line.chars().nth(0).unwrap() == '+'{
            houses.push(House::new(center, line_vec[0], line_vec[1], None));
            added_house = true;
        } else if line.chars().nth(0).unwrap() == '-'{
            let mut to_remove = 0;
            for (i, house) in houses.iter().enumerate(){
                if house == &House::new(center, line_vec[0], line_vec[1], None){
                    to_remove = i;
                    break
                }
            }
            houses.remove(to_remove);
        } else if line.chars().nth(0).unwrap() == '?'{
            let distances_from_corners = get_distances_from_corners((line_vec[0], line_vec[1]), wall_size);
            if added_house{
                houses.sort();
                added_house = false;
            }

            let max_distance = houses.last().unwrap().distance - remember_size as f64;

            let mut distances = vec![];

            for i in (0..houses.len()).rev(){
                if houses[i].distance < max_distance{
                    break;
                }
                distances.push(get_shortest_distance_to_entrance(houses[i], wall_size, &distances_from_corners));
            }

            println!("{}", distances.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap().round());
            // print!("\n");
        }
    }
}
