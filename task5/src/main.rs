use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let stdin = stdin();
    let mut input_line = String::new();
    stdin.read_line(&mut input_line).unwrap();
    let mut memory: Vec<isize> = input_line
        .trim()
        .split(",")
        .map(|n| n.parse())
        .map(|r| r.unwrap())
        .collect();
    execute_memory(&mut memory);
}

#[derive(Debug, PartialEq, Hash)]
enum Parameter {
    Position(usize),
    Immediate(isize),
}

impl Parameter {
    fn value(&self, memory: &[isize]) -> isize {
        match self {
            Parameter::Position(addr) => memory[*addr],
            Parameter::Immediate(val) => *val,
        }
    }

    fn address(&self) -> usize {
        match self {
            Parameter::Position(addr) => *addr,
            Parameter::Immediate(_) => panic!("Trying to get an address from an immediate parameter")
        }
    }

    fn immediate_value(&self) -> isize {
        match self {
            Parameter::Position(_) => panic!("Tying to get a value from an address parameter"),
            Parameter::Immediate(val) => *val,
        }
    }
}

struct Instruction {
    opcode: usize,
    parameters: Vec<Parameter>,
    location: usize,
}

impl Instruction {
    fn from_memory(memory: &[isize], location: usize) -> Instruction {
        let parameter_counts: HashMap<usize, usize> = [
            (1, 3),
            (2, 3),
            (3, 1),
            (4, 1),
            (5, 2),
            (6, 2),
            (7, 3),
            (8, 3),
            (99, 0),
        ]
        .iter()
        .cloned()
        .collect();
        let instruction_code = memory[location] as usize;
        let opcode = digit(instruction_code, 0) + digit(instruction_code, 1) * 10;
        let parameters = (1..(parameter_counts[&opcode] + 1)).into_iter().map(|arg| {
            match digit(instruction_code, (arg + 1) as u32) {
                0 => Parameter::Position(memory[location + arg] as usize),
                1 => Parameter::Immediate(memory[location + arg]),
                _ => unreachable!(),
            }
        });
        Instruction {
            opcode,
            location,
            parameters: parameters.collect(),
        }
    }

    fn size(&self) -> usize {
        self.parameters.len() + 1
    }

    fn execute(&self, memory: &[isize]) -> InstResult {
        // println!("{}, {:?}", self.opcode, self.parameters);
        match self.opcode {
            1 => InstResult::Write(
                self.parameters[0].value(memory) + self.parameters[1].value(memory),
                self.parameters[2].address(),
            ),
            2 => InstResult::Write(
                self.parameters[0].value(memory) * self.parameters[1].value(memory),
                self.parameters[2].address(),
            ),
            // Yeah, this is ugly and should probably be represented by another InstResult
            3 => {
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                let result: isize = input.trim().parse().unwrap();
                InstResult::Write(result, self.parameters[0].address())
            }
            4 => {
                println!("{}", self.parameters[0].value(memory));
                InstResult::NoOp
            }
            5 => {
                if self.parameters[0].value(memory) != 0 {
                    InstResult::ChangePc(self.parameters[1].value(memory) as usize)
                } else {
                    InstResult::NoOp
                }
            }
            6 => {
                if self.parameters[0].value(memory) == 0 {
                    InstResult::ChangePc(self.parameters[1].value(memory) as usize)
                } else {
                    InstResult::NoOp
                }
            }
            7 => {
                let value = self.parameters[0].value(memory) < self.parameters[1].value(memory);
                InstResult::Write(value as isize, self.parameters[2].address())
            }
            8 => {
                let value = self.parameters[0].value(memory) == self.parameters[1].value(memory);
                InstResult::Write(value as isize, self.parameters[2].address())
            }
            99 => InstResult::Halt,
            _ => unreachable!(),
        }
    }
}

enum InstResult {
    Write(isize, usize),
    ChangePc(usize),
    Halt,
    NoOp,
}

fn digit(x: usize, i: u32) -> usize {
    let ten = 10usize.pow(i);
    (x % (ten * 10)) / ten
}

fn execute_memory(memory: &mut [isize]) {
    let mut pc = 0;
    loop {
        let instruction = Instruction::from_memory(memory, pc);
        let mut increment_pc = true;
        match instruction.execute(memory) {
            InstResult::Write(result, address) => memory[address] = result,
            InstResult::ChangePc(address) => {
                increment_pc = false;
                pc = address;
            }
            InstResult::NoOp => (),
            InstResult::Halt => break,
        }
        if increment_pc {
            pc += instruction.size();
        }
    }
}

// fn find_inputs_for_output(initial_memory: Vec<isize>, output: isize) -> (isize, isize) {
//     let range = 100;
//     for i in 0..range {
//         for j in 0..range {
//             let mut memory = initial_memory.clone();
//             memory[1] = i;
//             memory[2] = j;
//             execute_memory(&mut memory);
//             if memory[0] == output {
//                 return (i, j)
//             }
//         }
//     }
//     unreachable!();
// }
