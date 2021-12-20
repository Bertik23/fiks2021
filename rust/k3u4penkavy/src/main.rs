// use std::fs::File;
// use std::io::BufReader;
// use std::io::prelude::*;
use std::collections::HashMap;
use std::fs;

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

fn is_within_levenshtein_distance(a: &String, b: &String, k: u8) -> bool{
    return true
}

fn main() {
    let bases_map: HashMap<String, &str> = HashMap::from([
        (String::from("00"), "A"),
        (String::from("01"), "C"),
        (String::from("10"), "G"),
        (String::from("11"), "T"),
    ]);

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

    for problem_n in 0..num_of_problems{
        println!("Problem {}/{}", problem_n, num_of_problems);
        line_vec = Vec::new();
        while input_bytes[byte_pointer] != 32 {
            line_vec.push(input_bytes[byte_pointer]);
            byte_pointer += 1;
        }
        byte_pointer += 1;
        let num_of_birds: u8 = String::from_utf8_lossy(&line_vec).parse().unwrap();

        let mut bird_genomes: Vec<String> = Vec::new();
        // let mut bird_same_species = HashMap::new();
        // bird_same_species.insert(0, vec![]);

        
        // println!("{}", byte_pointer);
        line_vec = bytes_until(&input_bytes, 10, &mut byte_pointer);
        let _max_genome_difference = line_vec_to_u8(&line_vec);
        // println!("{} {:?} {}", byte_pointer, line_vec, _max_genome_difference);
        for bird_n in 0..num_of_birds{
            println!("Bird {}/{}", bird_n+1, num_of_birds);
            line_vec = bytes_until(&input_bytes, 32, &mut byte_pointer);
            let num_of_bases: u64 = line_vec_to::<u64>(&line_vec);
            let num_of_bytes = (num_of_bases as f64/4.0).ceil() as u64;
            let u8_bases = n_bytes(&input_bytes, num_of_bytes, &mut byte_pointer);
            let binary_vec: String = u8_bases.into_iter().map(|i| zfill(format!("{:b}", i), 8 as usize)).collect();
            let binary_bases = &binary_vec[0..(num_of_bases*2) as usize];
            // println!("{:?}", binary_vec);
            let bases_str = bin_str_to_bases(&bases_map, binary_bases);
            println!("{:?}", bases_str);
            bird_genomes.push(bases_str);
        }
    }
}
