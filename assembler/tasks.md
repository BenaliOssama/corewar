1. Assembler

Goal: Convert .s assembly files into .cor binary files.
Subtasks:

Parser: Read .s files, split lines, identify labels, instructions, and parameters.

Label Resolver: Convert labels into relative byte offsets.

Binary Writer: Generate the binary .cor file with correct endianness, program size, name, description, and instructions.

Pcode & IDX Handling: Generate parameter code bytes and handle instructions with Has Idx.

Error Handling: Validate instructions, parameter types, and file format. Print errors without creating .cor if invalid.

Help Command: Show usage if no file or wrong input.

Testing: Feed example .s files (like ameba.s) and verify output with hexdump.
