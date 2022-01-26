fn zfill(in_string: String, length: usize) -> String{
    let mut out_string = String::from(in_string);
    while out_string.len() < length{
        out_string.insert(0, '0')
    }
    return out_string
}

struct Process{
    pc: usize,
    memory: [u32; 256]
}

impl Process{
    fn new(pc: u8, memory: Vec<u32>) -> Self{
        let mut mem: [u32; 256] = [0; 256];
        let mut load_pc = pc as usize;
        for instruction in memory.into_iter(){
            mem[load_pc] = instruction;
            load_pc += 1;
            load_pc = load_pc % 256;
        }
        Self{pc: pc as usize, memory: mem}
    }

    fn cycle(&self) -> bool{
        let instruction_argument = self.memory[self.pc];
        let bin_inst = zfill(format!("{:b}", instruction_argument), 32usize);
        let instruction = u8::from_str_radix(&String::from_utf8_lossy(&bin_inst.as_bytes()[24..32]), 2).unwrap();
        let argument = u8::from_str_radix(&String::from_utf8_lossy(&bin_inst.as_bytes()[0..24]), 2).unwrap();
        println!("inst: {} arg: {}", instruction, argument);
        true
    }
}

fn main() {
    let mut line = String::new();
    let mut _b1 = std::io::stdin().read_line(&mut line);
    let num_of_problems: u16 = line.parse().unwrap();
    for _problem_n in 0..num_of_problems{
        _b1 = std::io::stdin().read_line(&mut line);
        let num_of_processes: u8 = line.parse().unwrap();
        let mut processes = vec![];
        for _process_n in 0..num_of_processes{
            _b1 = std::io::stdin().read_line(&mut line);
            let line_vec = line.trim().split(" ").map(|p| p.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            processes.push(Process::new(line_vec[0] as u8, line_vec[1..].to_vec()))
        }
        for process in processes{
            process.cycle();
        }
    }
}
