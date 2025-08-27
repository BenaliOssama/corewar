

### 📘 Documentation on Corewar

* **Corewar overview (rules + VM concept)**:
  [https://corewar.co.uk/](https://corewar.co.uk/)
* **Corewar instruction set explanation**:
  [http://vyznev.net/corewar/guide.html](http://vyznev.net/corewar/guide.html)
* **Original Corewar ’94 spec (defines asm, VM, ops, cycles, etc.)**:
  [https://corewar.co.uk/standards/icws94.txt](https://corewar.co.uk/standards/icws94.txt)

---

### 📗 CPU & Architecture Basics

* **Von Neumann Architecture basics** (your VM design mimics this):
  [https://en.wikipedia.org/wiki/Von\_Neumann\_architecture](https://en.wikipedia.org/wiki/Von_Neumann_architecture)
* **What a Virtual Machine is (simple explanation)**:
  [https://en.wikipedia.org/wiki/Virtual\_machine](https://en.wikipedia.org/wiki/Virtual_machine)

---

### 📙 Assembler & Binary Format

* **How assemblers work (general idea)**:
  [https://en.wikipedia.org/wiki/Assembler\_(computing)](https://en.wikipedia.org/wiki/Assembler_%28computing%29)
* **Endianness explained (big-endian vs little-endian)**:
  [https://en.wikipedia.org/wiki/Endianness](https://en.wikipedia.org/wiki/Endianness)
* **How to write binary file formats** (important for `.cor` files):
  [https://wiki.osdev.org/Executable\_and\_Linkable\_Format](https://wiki.osdev.org/Executable_and_Linkable_Format)

---

### 📒 C Concepts (if you code in C)

* **Memory management (malloc, free – avoid leaks)**:
  [https://man7.org/linux/man-pages/man3/malloc.3.html](https://man7.org/linux/man-pages/man3/malloc.3.html)
* **Struct packing & binary writing** (for `.cor` file):
  [https://en.wikipedia.org/wiki/Data\_structure\_alignment](https://en.wikipedia.org/wiki/Data_structure_alignment)
* **File I/O in C (fread, fwrite, open)**:
  [https://man7.org/linux/man-pages/man3/fwrite.3.html](https://man7.org/linux/man-pages/man3/fwrite.3.html)

---

### 📕 Debugging & Testing

* **Hexdump usage** (inspect `.cor` binaries):
  [https://man7.org/linux/man-pages/man1/hexdump.1.html](https://man7.org/linux/man-pages/man1/hexdump.1.html)
* **GDB basics (to debug your VM)**:
  [https://beej.us/guide/bggdb/](https://beej.us/guide/bggdb/)

---

### 🛠️ Playground & References

* GitHub repos with working **Corewar Assembler + VM** (study them, don’t just copy):

  * [https://github.com/FlorianCassayre/corewar](https://github.com/FlorianCassayre/corewar)
  * [https://github.com/avl/COREWAR](https://github.com/avl/COREWAR)
  * [https://github.com/hogiahien/corewar](https://github.com/hogiahien/corewar)

---

### ✅ Strategy to Learn

1. **Start with the Assembler**

   * Parse `.s` → validate syntax → output `.cor` (binary).
   * Practice with small `.s` examples and check your `.cor` with `hexdump`.

2. **Build the Virtual Machine**

   * Load `.cor` players into arena (memory).
   * Implement instruction execution cycle by cycle.
   * Handle PC (program counter), registers, carry flag.
   * Implement live, zjmp, add, sub first, then expand.

3. **Test with given players (like `ameba.s`)**

   * Use provided reference VM (`-v` flag) to compare with your own VM step by step.

4. **Once stable, write your own player**.

