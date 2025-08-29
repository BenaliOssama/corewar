2. Virtual Machine (VM)

Goal: Execute .cor files in a circular memory arena.
Subtasks:

Memory Arena: Circular memory with configurable size.

Process Management: Track PC, registers, carry flag, and fork new processes.

Instruction Execution: Implement each instruction (live, ld, st, add, etc.) with cycles and parameter handling.

Cycle Management: Execute instructions over cycles and check CYCLE_TO_DIE conditions.

Player Loading: Load up to 4 players evenly spaced.

End Game Detection: Check which processes are alive, declare winner.

Dump Feature: Implement -d [NB_CYCLES] to dump memory state.

Error Handling: Handle corrupted .cor files gracefully.

Greeting and Result Messages: Print start and end messages correctly.

Testing: Use reference .cor files and compare output.
