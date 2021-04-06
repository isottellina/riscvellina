# Riscvellina

This is a work-in-progress RISC-V (RV64G) emulator. The current aim is to be able to boot vx6.
To launch it with a binary file, just run :

    cargo run -- bin-file

To make a bin file, just write a xxx.s file with RISC-V assembly and run :

    make xxx.bin