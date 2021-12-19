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

fn bin_str_to_bases(bases_map: &HashMap<String, &str>, in_string: &str) -> String{
    let mut bases_str = String::new();
    for i in 0..in_string.len()/2 {
        // println!("{:?}", &in_string[i*2..i*2+2]);
        // println!("{:?}", bases_map.get(&in_string[i*2..i*2+2]));
        bases_str.push_str(bases_map.get(&in_string[i*2..i*2+2]).unwrap());
    }
    return bases_str;
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
    let num_of_problems: u32 = line.replace("\r\n", "").parse().unwrap();

    println!("{}", num_of_problems);
    for problem_n in 0..num_of_problems{
        println!("Problem {}/{}", problem_n, num_of_problems);
        let mut line = String::new();
        _b1 = std::io::stdin().read_line(&mut line);
        let input = line.split(" ").collect::<Vec<&str>>();
        let num_of_birds: u8 = input[0].parse().unwrap();
        let _max_genome_difference: u8 = input[1].replace("\r\n", "").parse().unwrap();
        for bird_n in 0..num_of_birds{
            println!("Bird {}/{}", bird_n, num_of_birds);
            let mut line = String::new();
            _b1 = std::io::stdin().read_line(&mut line);
            let input = line.split(" ").collect::<Vec<&str>>();
            println!("{}", input[0]);
            let num_of_bases: u64 = input[0].parse().unwrap();
            let num_of_bytes = (num_of_bases as f64/4.0).ceil() as usize;
            let ascii_bases = &input[1][0..num_of_bytes];
            let binary_vec: String = ascii_bases.as_bytes().into_iter().map(|i| zfill(format!("{:b}", i), 8 as usize)).collect();
            let binary_bases = &binary_vec[0..(num_of_bases*2) as usize];
            println!("{:?}", binary_vec);
            println!("{:?}", bin_str_to_bases(&bases_map, binary_bases))
        }
    }

    //let f = File::open("controlInput.txt").expect("Unable to open");


    // let mut buf_reader = BufReader::new(f);
    // let _b2 = buf_reader.read_to_string(&mut line);
    println!("Text: {}, bytes: {:?}", line, line.as_bytes());
}
