// use std::fs::File;
// use std::io::BufReader;
// use std::io::prelude::*;
// use std::io;
use std::collections::HashMap;
use std::fs;
use std::cmp::min;
use itertools::Itertools;
// use num_integer::binomial;
use std::mem;

fn zfill(in_string: String, length: usize) -> String{
    let mut out_string = String::from(in_string);
    while out_string.len() < length{
        out_string.insert(0, '0')
    }
    return out_string
}

fn bytes_until(input_bytes: &[u8], until: u8, byte_pointer: &mut usize) -> Vec<u8>{
    let mut line_vec: Vec<u8> = Vec::new();
    while input_bytes[*byte_pointer] != until {
        line_vec.push(input_bytes[*byte_pointer]);
        *byte_pointer += 1;
    }
    *byte_pointer += 1;
    return line_vec;
}

fn n_bytes(input_bytes: &[u8], number: u64, byte_pointer: &mut usize) -> Vec<u8>{
    let mut line_vec: Vec<u8> = Vec::new();
    for _ in 0..number {
        line_vec.push(input_bytes[*byte_pointer]);
        *byte_pointer += 1;
    }
    *byte_pointer += 1;
    return line_vec;
}

fn bin_str_to_bases(bases_map: &HashMap<String, &str>, in_string: &str) -> String{
    let mut bases_str = String::new();
    for i in 0..in_string.len()/2 {
        // println!("{:?}", &in_string[i*2..i*2+2]);
        // println!("{:?}", bases_map.get(&in_string[i*2..i*2+2]));
        bases_str.push_str(bases_map.get(&in_string[i*2..i*2+2]).unwrap());
    }
    return bases_str;
}

fn letter_counts(input_string: &String) -> HashMap<char, i32>{
    let mut letter_counts: HashMap<char, i32> = HashMap::new();
    letter_counts.insert('a', 0);
    letter_counts.insert('g', 0);
    letter_counts.insert('c', 0);
    letter_counts.insert('t', 0);

    // let input_string = "Hello, world!";
    let char_vec: Vec<char> = input_string.to_lowercase().chars().collect();
    for c in char_vec {
        *letter_counts.entry(c).or_insert(0) += 1;
    }
    // println!("{:?}",letter_counts);
    return letter_counts;
}

fn line_vec_to_u8(line_vec: &[u8]) -> u8{
    let out: u8 = String::from_utf8_lossy(&line_vec).parse().unwrap();
    return out;
}

#[allow(unused)]
fn line_vec_to_u128(line_vec: &[u8]) -> u128{
    let out: u128 = String::from_utf8_lossy(&line_vec).parse().unwrap();
    return out;
}

fn line_vec_to<T>(line_vec: &[u8]) -> T
where
T: std::str::FromStr,
<T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let out: T = String::from_utf8_lossy(&line_vec).parse::<T>().unwrap();
    return out;
}

#[allow(unused)]
fn is_within_levenshtein_distance(a: &String, b: &String, k: u8) -> bool{
    if (a.as_bytes().len() as i32 - b.as_bytes().len() as i32).abs() > k.into(){
        return false;
    }
    let a_counts = letter_counts(a);
    let b_counts = letter_counts(b);
    // let mut counts_diff_sum = 0;
    for ch in a_counts.keys(){
        // counts_diff_sum += (b_counts[ch] - a_counts[ch]).abs();
        // if counts_diff_sum > k.into(){
        if (b_counts[ch] - a_counts[ch]).abs() > k.into(){
            return false;
        }
    }
    // create two work vectors of integer distances
    let mut cur_row = vec![];
    let mut prev_row = vec![];

    let mut deletion_cost;
    let mut insertion_cost;
    let mut substitution_cost;

    // initialize v0 (the previous row of distances)
    // this row is A[0][i]: edit distance for an empty s
    // the distance is just the number of characters to delete from t
    for i in 0..=b.len(){
        prev_row.push(i);
        cur_row.push(0);
    }

    for i in 0..a.len(){
        // calculate v1 (current row distances) from the previous row v0

        // first element of v1 is A[i + 1][0]
        //   edit distance is delete (i + 1) chars from s to match empty t
        cur_row[0] = i + 1;

        // use formula to fill in the rest of the row
        for j in 0..b.len(){
            // calculating costs for A[i + 1][j + 1]
            deletion_cost = prev_row[j + 1] + 1;
            insertion_cost = cur_row[j] + 1;
            if a.as_bytes()[i] == b.as_bytes()[j]{
                substitution_cost = prev_row[j];
            } else {
                substitution_cost = prev_row[j] + 1;
            }

            cur_row[j + 1] = min(deletion_cost, min(insertion_cost, substitution_cost));
        }
        if cur_row.iter().min().unwrap() > &k.into(){
            return false
        }

        // copy v1 (current row) to v0 (previous row) for next iteration
        // since data in v1 is always invalidated, a swap without copy could be more efficient
        // println!("{:?}", cur_row);
        // prev_row = cur_row[0..cur_row.len()].to_vec();
        mem::swap(&mut cur_row, &mut prev_row)
    // after the last swap, the results of v1 are now in v0
    }
    return prev_row[b.len()] <= k.into();
}

#[allow(unused)]
fn levenshtein_distance(a: &String, b: &String) -> usize{
    if b.len() == 0{
        return a.len();
    }
    if a.len() == 0{
        return b.len();
    }
    if a.as_bytes()[0] == b.as_bytes()[0]{
        return levenshtein_distance(&String::from(&a[1..a.len()]), &String::from(&b[1..b.len()]));
    }
    return 1 + min(
        levenshtein_distance(a, &String::from(&b[1..b.len()])),
        min(
            levenshtein_distance(&String::from(&a[1..a.len()]), b),
            levenshtein_distance(&String::from(&a[1..a.len()]), &String::from(&b[1..b.len()]))
        )
    )
}

#[allow(unused)]
fn levenshtein_distance_better(a: &String, b: &String) -> usize{
    let mut d = vec![];
    for i in 0..=a.len(){
        d.push(vec![]);
        for j in 0..=b.len(){
            d[i].push(0);
        }
    }
    
    for j in 0..=b.len(){
        d[0][j] = j;
    }
    for i in 0..=a.len(){
        d[i][0] = i;
    }
    
    let mut substitution_cost;
    for j in 1..=b.len(){
        for i in 1..=a.len(){
            if a.as_bytes()[i-1] == b.as_bytes()[j-1]{
                substitution_cost = 0
            } else{
                substitution_cost = 1
            }
            
            d[i][j] = min(
                d[i-1][j] + 1,
                min(
                    d[i][j-1] + 1,
                    d[i-1][j-1] + substitution_cost
                )
            )
        }
    }
    // dbg!(&d);
    return d[a.len()][b.len()]
}

#[allow(unused)]
fn levenshtein_distance_better_better(a: &String, b: &String) -> usize{
    // create two work vectors of integer distances
    let mut cur_row = vec![];
    let mut prev_row = vec![];
    
    let mut deletion_cost;
    let mut insertion_cost;
    let mut substitution_cost;
    
    // initialize v0 (the previous row of distances)
    // this row is A[0][i]: edit distance for an empty s
    // the distance is just the number of characters to delete from t
    for i in 0..=b.len(){
        prev_row.push(i);
        cur_row.push(0);
    }

    for i in 0..a.len(){
        // calculate v1 (current row distances) from the previous row v0

        // first element of v1 is A[i + 1][0]
        //   edit distance is delete (i + 1) chars from s to match empty t
        cur_row[0] = i + 1;

        // use formula to fill in the rest of the row
        for j in 0..b.len(){
            // calculating costs for A[i + 1][j + 1]
            deletion_cost = prev_row[j + 1] + 1;
            insertion_cost = cur_row[j] + 1;
            if a.as_bytes()[i] == b.as_bytes()[j]{
                substitution_cost = prev_row[j];
            } else {
                substitution_cost = prev_row[j] + 1;
            }

            cur_row[j + 1] = min(deletion_cost, min(insertion_cost, substitution_cost));
        }

        // copy v1 (current row) to v0 (previous row) for next iteration
        // since data in v1 is always invalidated, a swap without copy could be more efficient
        prev_row = cur_row[0..cur_row.len()].to_vec();
    // after the last swap, the results of v1 are now in v0
    }
    return prev_row[b.len()]
}

fn main() {
    let bases_map: HashMap<String, &str> = HashMap::from([
        (String::from("00"), "A"),
        (String::from("01"), "C"),
        (String::from("10"), "G"),
        (String::from("11"), "T"),
    ]);

    // let x = (0..2000).map(|_| "a").collect::<String>();
    // let y = (0..2000).map(|_| "b").collect::<String>();
    // let start = Instant::now();
    // let dist = levenshtein_distance_better_better(&x, &y);
    // println!("{} milliseconds\nDistance: {}", start.elapsed().as_millis(), dist);
    // let start = Instant::now();
    // let dist = levenshtein_distance_better(&x, &y);
    // println!("{} milliseconds\nDistance: {}", start.elapsed().as_millis(), dist);


    let mut line = String::from("");
    // print!("Input: ");
    let mut _b1 = std::io::stdin().read_line(&mut line);
    let input_bytes = fs::read(line.replace("\r\n", "")).unwrap();
    let mut byte_pointer: usize = 0;
    let mut line_vec: Vec<u8> = Vec::new();
    while input_bytes[byte_pointer] != 10 {
        line_vec.push(input_bytes[byte_pointer]);
        byte_pointer += 1;
    }
    byte_pointer += 1;

    let num_of_problems: u8 = String::from_utf8_lossy(&line_vec).parse().unwrap();
    // println!("{}", num_of_problems);

    for _problem_n in 0..num_of_problems{
        // println!("Problem {}/{}", problem_n+1, num_of_problems);
        line_vec = Vec::new();
        while input_bytes[byte_pointer] != 32 {
            line_vec.push(input_bytes[byte_pointer]);
            byte_pointer += 1;
        }
        byte_pointer += 1;
        let num_of_birds: u8 = String::from_utf8_lossy(&line_vec).parse().unwrap();

        let mut bird_genomes: Vec<String> = Vec::new();
        let mut bird_same_species = HashMap::new();

        
        // println!("{}", byte_pointer);
        line_vec = bytes_until(&input_bytes, 10, &mut byte_pointer);
        let max_genome_difference = line_vec_to_u8(&line_vec);
        // println!("{} {:?} {}", byte_pointer, line_vec, _max_genome_difference);
        for bird_n in 0..num_of_birds{
            bird_same_species.insert(bird_n, vec![]);
            // println!("Bird {}/{}", bird_n+1, num_of_birds);
            line_vec = bytes_until(&input_bytes, 32, &mut byte_pointer);
            let num_of_bases: u64 = line_vec_to::<u64>(&line_vec);
            let num_of_bytes = (num_of_bases as f64/4.0).ceil() as u64;
            let u8_bases = n_bytes(&input_bytes, num_of_bytes, &mut byte_pointer);
            let binary_vec: String = u8_bases.into_iter().map(|i| zfill(format!("{:b}", i), 8 as usize)).collect();
            let binary_bases = &binary_vec[0..(num_of_bases*2) as usize];
            // println!("{:?}", binary_vec);
            let bases_str = bin_str_to_bases(&bases_map, binary_bases);
            // println!("{:?}", bases_str);
            bird_genomes.push(bases_str);
        }
        // println!("Going for the combinations");
        // let combs = binomial(bird_genomes.len(), 2);
        for (_i, comb) in (0..=bird_genomes.len() as u8 - 1).combinations(2).enumerate(){
            // print!("Combination {}/{}\r", i+1, combs);
            // io::stdout().flush().unwrap();
            // if levenshtein_distance_better_better(&bird_genomes[comb[0] as usize], &bird_genomes[comb[1] as usize]) <= max_genome_difference.into(){
            if is_within_levenshtein_distance(&bird_genomes[comb[0] as usize], &bird_genomes[comb[1] as usize], max_genome_difference){
                bird_same_species.get_mut(&comb[0]).unwrap().push(comb[1]);
                bird_same_species.get_mut(&comb[1]).unwrap().push(comb[0]);
            }
        }
        // println!("Going for the triplets");
        // println!("{:?}", bird_same_species);
        // println!("{}", is_within_levenshtein_distance(&bird_genomes[1], &bird_genomes[2], 2));
        // println!("{}", levenshtein_distance(&bird_genomes[1], &bird_genomes[2]));
        let mut triplets = vec![];
        for bird in bird_same_species.keys(){
            for bird_ss in bird_same_species.get(bird).unwrap(){
                for bird_ss_ss in bird_same_species.get(bird_ss).unwrap(){
                    if bird_ss_ss < bird{
                        continue;
                    }
                    if !bird_same_species.get(bird_ss_ss).unwrap().contains(bird) && bird != bird_ss_ss{
                        triplets.push((bird, bird_ss, bird_ss_ss));
                    }
                }
            }
        }
        println!("{}", triplets.len());
        for triplet in triplets{
            println!("{} {} {}", triplet.0, triplet.1, triplet.2);
        }
        // dbg!(levenshtein_distance(&bird_genomes[0], &bird_genomes[bird_genomes.len()-1]));
        // dbg!(levenshtein_distance_better(&bird_genomes[0], &bird_genomes[bird_genomes.len()-1]));
    }
}
