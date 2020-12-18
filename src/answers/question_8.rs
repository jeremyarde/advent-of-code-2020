use std::fs;

#[derive(Debug)]
enum OpCode {
    Nop,
    Acc,
    Jmp,
}

fn main() {
    let filename = "inputs/q8_input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    let mut accumulator = 0;
    let mut curr_instruction = 0;

    let mut instructions: Vec<(OpCode, i32)> = vec![];
    let mut instruction_run_count: Vec<i32> = vec![];

    for line in contents.lines() {
        instruction_run_count.push(0);
        let info = line.split(' ').collect::<Vec<&str>>();
        let opcode = info[0];
        let value: i32 = info[1].parse().unwrap();
        match opcode {
            "nop" => instructions.push((OpCode::Nop, value)),
            "acc" => instructions.push((OpCode::Acc, value)),
            "jmp" => instructions.push((OpCode::Jmp, value)),
            _ => {}
        }
    }

    loop {
        let slice_location = curr_instruction as usize;

        instruction_run_count[slice_location] += 1;
        if instruction_run_count[slice_location] >= 2 {
            break;
        }

        match instructions[slice_location].0 {
            OpCode::Nop => curr_instruction += 1,
            OpCode::Acc => {
                curr_instruction += 1;
                accumulator += instructions[slice_location].1
            }
            OpCode::Jmp => curr_instruction += instructions[slice_location].1,
        }
    }
    println!("Accumulator value: {:?}", accumulator);
}
