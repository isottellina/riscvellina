# Riscvellina

This is a work-in-progress RISC-V (RV64G) emulator. The current aim is to be able to boot xv6.
To launch it with a binary file, just run :

    cargo run -- bin-file

To make a bin file, just write a xxx.s file with RISC-V assembly and run :

    make xxx.bin

Currently, the emulator only prints the state of its CPU at each instruction, and stops at a NOP (addi, x0, x0, 0). You have to remember that. It will soon be corrected.

# TODO (for now)
 - Debugger (now there's only a disassembler)
 - UART
 - Ziscr extension
 - RV64A extension