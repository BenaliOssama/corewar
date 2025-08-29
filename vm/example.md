## 1. Define the Core Structures

You’ll need **three main components**:

### A. Arena (Memory)

```rust
const ARENA_SIZE: usize = 4096;

struct Arena {
    memory: [u8; ARENA_SIZE],
}

impl Arena {
    fn new() -> Self {
        Arena { memory: [0; ARENA_SIZE] }
    }

    fn read(&self, addr: usize, size: usize) -> Vec<u8> {
        (0..size).map(|i| self.memory[(addr + i) % ARENA_SIZE]).collect()
    }

    fn write(&mut self, addr: usize, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.memory[(addr + i) % ARENA_SIZE] = byte;
        }
    }
}
```

* Circular memory is handled using modulo.
* `read` and `write` handle wrapping around the arena.

---

### B. Process (like a CPU thread)

```rust
struct Process {
    pc: usize,
    regs: [i32; 16],
    carry: bool,
    id: i32,
    wait_cycles: usize, // cycles before instruction executes
}

impl Process {
    fn new(id: i32, start_pc: usize) -> Self {
        let mut regs = [0; 16];
        regs[0] = -id; // r1 = -player_id
        Process { pc: start_pc, regs, carry: false, id, wait_cycles: 0 }
    }
}
```

* Each player starts with **one process**.
* `wait_cycles` allows you to simulate instruction delays.

---

### C. VM (Virtual Computer)

```rust
struct VM {
    arena: Arena,
    processes: Vec<Process>,
    cycle: usize,
}

impl VM {
    fn new() -> Self {
        VM { arena: Arena::new(), processes: Vec::new(), cycle: 0 }
    }

    fn add_process(&mut self, process: Process) {
        self.processes.push(process);
    }

    fn run(&mut self, max_cycles: usize) {
        while !self.processes.is_empty() && self.cycle < max_cycles {
            self.cycle += 1;
            let mut new_processes = vec![];

            for process in &mut self.processes {
                if process.wait_cycles > 0 {
                    process.wait_cycles -= 1;
                    continue;
                }

                let opcode = self.arena.read(process.pc, 1)[0];
                match opcode {
                    0x01 => self.execute_live(process), // just an example
                    _ => process.pc = (process.pc + 1) % ARENA_SIZE,
                }
            }

            self.processes.extend(new_processes);
        }
    }

    fn execute_live(&self, process: &mut Process) {
        println!("Player {} says it is alive!", process.id);
        process.pc = (process.pc + 5) % ARENA_SIZE; // live instruction = 1 opcode + 4 bytes
    }
}
```

---

## 2. First Steps to Get It Working

1. **Load one `.cor` player** into the arena.
2. **Create a process** for it starting at the player’s offset.
3. **Run the VM loop** with `cycle = 0..max_cycles`.
4. **Implement `live` first** → verify the loop works.

Once this works, **expand to other instructions** (`ld`, `st`, `add`, etc.) one at a time.

---

## 3. Tips for Incremental Development

* Start with **just one player** to debug arena + process logic.
* Print the **PC, opcode, and cycle** at each step to trace execution.
* Only after `live` works, implement instructions that modify **registers and arena**.
* Ignore pcode and IDX\_MOD at first; add them once basic execution works.

