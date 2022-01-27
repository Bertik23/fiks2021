use std::collections::{HashMap, HashSet};


fn zfill(in_string: String, length: usize) -> String{
    let mut out_string = String::from(in_string);
    while out_string.len() < length{
        out_string.insert(0, '0')
    }
    return out_string
}

fn parse_instruction(instruction_argument: u32) -> Option<(u8, String, u32)>{
    let instruction_set = HashMap::from([
        (0, "NOP"),
        (1, "PC"),
        (2, "PUSH"),
        (3, "POP"),
        (4, "SWAP"),
        (5, "DUP"),
        (6, "PUSHSSZ"),
        (7, "LOAD"),
        (8, "STORE"),
        (9, "ADD"),
        (10, "SUB"),
        (11, "DIV"),
        (12, "POW"),
        (13, "BRZ"),
        (14, "BR3"),
        (15, "BR7"),
        (16, "BRGE"),
        (17, "JMP"),
        (18, "ARMED_BOMB"),
        (19, "BOMB"),
        (20, "TLPORT"),
        (21, "JNTAR")
    ]);
    let bin_inst = zfill(format!("{:b}", instruction_argument), 32usize);
    let instruction = u8::from_str_radix(&String::from_utf8_lossy(&bin_inst.as_bytes()[24..32]), 2).unwrap();
    let instruction_str = instruction_set.get(&instruction);
    if instruction_str == None{
        return None
    }
    // println!("{:?} bytes: {:?}, string: {}", bin_inst, &bin_inst.as_bytes()[0..24], &String::from_utf8_lossy(&bin_inst.as_bytes()[0..24]));
    let argument = u32::from_str_radix(&String::from_utf8_lossy(&bin_inst.as_bytes()[0..24]), 2).unwrap();
    return Some((instruction, instruction_str.unwrap().to_string(), argument))
}


struct Process{
    pc: usize,
    stack: Vec<u32>,
    alive: bool,
    waiting_for_tp: bool
}

impl Process{
    fn new(pc: u8, memory: Vec<u32>, main_memory: &mut [u32], instructions_hashset: &mut HashSet<String>) -> Self{
        //let mem: [u32; 256] = [0; 256];
        let mut load_pc = pc as usize;
        for instruction in memory.into_iter(){
            if load_pc != 0{
                main_memory[load_pc] = instruction;
            }
            let parsed = parse_instruction(instruction).unwrap_or_default();
            instructions_hashset.insert(parsed.1);
            // eprintln!("{} {}", parsed.1, parsed.2);
            load_pc += 1;
            load_pc = load_pc % 256;
        }
        Self{pc: pc as usize, stack: vec![], alive: true, waiting_for_tp: false}
    }

    fn cycle(&mut self, main_memory: &mut [u32]) -> bool{
        if self.waiting_for_tp{
            return true
        }
        let instruction_argument = main_memory[self.pc];
        // let bin_inst = zfill(format!("{:b}", instruction_argument), 32usize);
        // let instruction = u8::from_str_radix(&String::from_utf8_lossy(&bin_inst.as_bytes()[24..32]), 2).unwrap();
        // let instruction_str = instruction_set.get(&instruction).unwrap();
        // // println!("{:?} bytes: {:?}, string: {}", bin_inst, &bin_inst.as_bytes()[0..24], &String::from_utf8_lossy(&bin_inst.as_bytes()[0..24]));
        // let argument = u32::from_str_radix(&String::from_utf8_lossy(&bin_inst.as_bytes()[0..24]), 2).unwrap();
        let possible_parse = parse_instruction(instruction_argument);
        if possible_parse == None{
            return false
        }
        let (_instruction, instruction_str, argument) = possible_parse.unwrap();
        // println!(
        //     "inst: {} instID: {}\narg: {}\npc: {}",
        //     instruction_set.get(&instruction).unwrap(), instruction,
        //     argument,
        //     self.pc
        //     //main_memory
        // );
        // println!("{:?}", (self.pc, _instruction, &instruction_str, &argument));
        if instruction_str == "NOP"{
        }
        else if instruction_str == "PC"{
            self.stack.push(self.pc as u32);
        } else if instruction_str == "PUSH"{
            self.stack.push(argument);
        } else if instruction_str == "POP"{
            let a = self.stack.pop();
            if a == None{
                return false
            }
        } else if instruction_str == "SWAP"{
            let a = self.stack.pop();
            let b = self.stack.pop();
            if a == None || b == None{
                return false
            }
            self.stack.push(a.unwrap());
            self.stack.push(b.unwrap());
        } else if instruction_str == "DUP"{
            if self.stack.last() == None{
                return false
            }
            self.stack.push(*self.stack.last().unwrap());
        } else if instruction_str == "PUSHSSZ"{
            self.stack.push(self.stack.len() as u32);
        } else if instruction_str == "LOAD"{
            let a = self.stack.pop();
            if a == None || a.unwrap() == 666{
                return false
            }
            self.stack.push(main_memory[(a.unwrap() % 256) as usize]);
        } else if instruction_str == "STORE"{
            let a = self.stack.pop();
            let b = self.stack.pop();
            if a == None || b == None || a.unwrap() == 666{
                return false
            }
            main_memory[(a.unwrap() % 256) as usize] = b.unwrap();
        } else if instruction_str == "ADD"{
            let a = self.stack.pop();
            let b = self.stack.pop();
            if a == None || b == None{
                return false
            }
            self.stack.push(a.unwrap().wrapping_add(b.unwrap()));
        } else if instruction_str == "SUB"{
            let a = self.stack.pop();
            let b = self.stack.pop();
            if a == None || b == None{
                return false
            }
            self.stack.push(a.unwrap().wrapping_sub(b.unwrap()));
        } else if instruction_str == "DIV"{
            let a = self.stack.pop();
            let b = self.stack.pop();
            if a == None || b == None{
                return false
            }
            if b.unwrap() == 0{
                return false
            }
            self.stack.push(a.unwrap().wrapping_div(b.unwrap()));
        } else if instruction_str == "POW"{
            let a = self.stack.pop();
            let b = self.stack.pop();
            if a == None || b == None{
                return false
            }
            self.stack.push(a.unwrap().wrapping_pow(b.unwrap()));
        } else if instruction_str == "BRZ"{
            let a = self.stack.pop();
            if a == None{
                return false
            }
            if a.unwrap() == 0{
                self.pc += argument as usize;
            }
        } else if instruction_str == "BR3"{
            let a = self.stack.pop();
            if a == None{
                return false
            }
            if a.unwrap() == 3{
                self.pc += argument as usize;
            }
        } else if instruction_str == "BR7"{
            let a = self.stack.pop();
            if a == None{
                return false
            }
            if a.unwrap() == 7{
                self.pc += argument as usize;
            }
        } else if instruction_str == "BRGE"{
            let a = self.stack.pop();
            let b = self.stack.pop();
            if a == None || b == None{
                return false
            }
            if a.unwrap() >= b.unwrap(){
                self.pc += argument as usize;
            }
        } else if instruction_str == "JMP"{
            self.pc = argument as usize;
        } else if instruction_str == "ARMED_BOMB"{
            return false;
        } else if instruction_str == "BOMB"{
            main_memory[self.pc] -= 1;
        } else if instruction_str == "TLPORT"{
            self.waiting_for_tp = true;
            return true
        } else if instruction_str == "JNTAR"{
            for i in 1..=3{
                let a = self.pc.wrapping_add(2usize.pow(i));
                let b = self.pc.wrapping_sub(2usize.pow(i));
                if a == 666 || b == 666 {
                    return false
                }
                main_memory[a % 256] = 19;
                main_memory[b % 256] = 19;
            }
        } else {
            return false
        }
        if self.stack.len() > 16{
            return false
        }
        self.pc += 1;
        self.pc = self.pc % 256;
        true
    }
}


fn main() {
    let mut line = String::new();
    let mut _b1 = std::io::stdin().read_line(&mut line);
    let num_of_problems: u16 = line.trim().parse().unwrap();
    let mut instructions_hashset: HashSet<String> = HashSet::new();
    for problem_n in 0..num_of_problems{
        if problem_n == 382{
            eprintln!("{:?}", instructions_hashset)
        }
        if problem_n == 490{
            eprintln!("{:?}", instructions_hashset)
        }
        line.clear();
        _b1 = std::io::stdin().read_line(&mut line);
        // eprintln!("Line: {}\nBytes: {:?}", line, line.as_bytes());
        let num_of_processes: u16 = line.trim().parse().unwrap();
        let mut processes = vec![];
        let mut main_memory: [u32; 256] = [0u32; 256];
        for _process_n in 0..num_of_processes{
            line.clear();
            _b1 = std::io::stdin().read_line(&mut line);
            let line_vec = line.trim().split(" ").map(|p| p.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            processes.push(Process::new(line_vec[0] as u8, line_vec[2..].to_vec(), &mut main_memory, &mut instructions_hashset));
        }
        let mut waiting_for_tp = vec![];
        for _c in 0..5000{
            for p in 0..processes.len(){
                waiting_for_tp.clear();
                if !processes[p].alive{
                    continue
                }
                if !processes[p].cycle(&mut main_memory){
                    processes[p].alive = false;
                }
                if processes[p].waiting_for_tp{
                    waiting_for_tp.push(p);
                }
                if waiting_for_tp.len() >= 2{
                    let first = processes[waiting_for_tp[0]].pc;
                    for (i, pj) in waiting_for_tp.iter().enumerate(){
                        if i+1 < waiting_for_tp.len(){
                            processes[*pj].pc = processes[waiting_for_tp[i+1]].pc + 1;
                        } else {
                            processes[*pj].pc = first + 1;
                        }
                        processes[*pj].pc = processes[*pj].pc % 256;
                        processes[*pj].waiting_for_tp = false;
                    }
                }
            }
        }
        let mut sum_pc = 0;
        for p in 0..processes.len(){
            sum_pc += processes[p].pc;
        }
        println!("{} {}", main_memory[42], sum_pc);
    }
}
