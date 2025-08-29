#[derive(Debug)]
struct Process {
    pc: usize,              // Program Counter
    registers: [i32; 16],   // Registers r1..r16
    carry: bool,            // Carry flag
}

const MEMORY_SIZE: usize = 256;
type Memory = [u8; MEMORY_SIZE];

// Opcodes
const OPCODE_LIVE: u8 = 0x01;
const OPCODE_ADD: u8 = 0x04;

// Simple CPU execution function
fn execute_instruction(process: &mut Process, memory: &mut Memory) {
    let opcode = memory[process.pc];
    match opcode {
        OPCODE_LIVE => {
            println!("Process at PC {} executed LIVE", process.pc);
            process.pc += 5; // 1 opcode + 4 bytes parameter
        }
        OPCODE_ADD => {
            // Example: add r1 + r2 -> r3
            let r1 = memory[process.pc + 1] as usize - 1;
            let r2 = memory[process.pc + 2] as usize - 1;
            let r3 = memory[process.pc + 3] as usize - 1;

            process.registers[r3] =
                process.registers[r1] + process.registers[r2];

            // Update carry flag
            process.carry = process.registers[r3] == 0;

            println!(
                "ADD: r{} + r{} -> r{} = {}",
                r1 + 1,
                r2 + 1,
                r3 + 1,
                process.registers[r3]
            );

            process.pc += 4; // opcode + 3 params
        }
        _ => {
            println!("Unknown opcode {}", opcode);
            process.pc += 1;
        }
    }
}

fn main() {
    let mut memory = [0u8; MEMORY_SIZE];
    let mut process = Process {
        pc: 0,
        registers: [0; 16],
        carry: false,
    };

    // Hardcode a small program
    memory[0] = OPCODE_ADD; // add r1 + r2 -> r3
    memory[1] = 1;
    memory[2] = 2;
    memory[3] = 3;

    memory[4] = OPCODE_LIVE; // live instruction

    // Execute instructions
    while process.pc < 5 {
        execute_instruction(&mut process, &mut memory);
    }

    println!("Final process state: {:?}", process);
}

