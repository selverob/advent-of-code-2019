use std::io::stdin;

fn main() {
    let stdin = stdin();
    let mut input_line = String::new();
    stdin.read_line(&mut input_line).unwrap();
    let mut memory: Vec<usize> = input_line
        .trim()
        .split(",")
        .map(|n| n.parse())
        .map(|r| r.unwrap())
        .collect();
    //execute_memory(&mut memory);
    let (x, y) = find_inputs_for_output(memory, 19690720);
    println!("{} {}", x, y);
}

enum OpResult {
    Write(usize, usize),
    Halt,
}

fn execute_memory(memory: &mut [usize]) {
    let mut pc = 0;
    loop {
        match interpret_opcode(&memory, pc) {
            OpResult::Write(result, address) => {
                memory[address] = result;
                pc += 4
            }
            OpResult::Halt => break,
        }
    }
}

fn interpret_opcode(memory: &[usize], pc: usize) -> OpResult {
    match memory[pc] {
        1 => OpResult::Write(memory[memory[pc + 1]] + memory[memory[pc + 2]], memory[pc + 3]),
        2 => OpResult::Write(memory[memory[pc + 1]] * memory[memory[pc + 2]], memory[pc + 3]),
        99 => OpResult::Halt,
        _ => unreachable!(),
    }
}

fn find_inputs_for_output(initial_memory: Vec<usize>, output: usize) -> (usize, usize) {
    let range = 100;
    for i in 0..range {
        for j in 0..range {
            let mut memory = initial_memory.clone();
            memory[1] = i;
            memory[2] = j;
            execute_memory(&mut memory);
            if memory[0] == output {
                return (i, j)
            }
        }
    }
    unreachable!();
}
