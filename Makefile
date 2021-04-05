%.bin: %.elf
	riscv64-elf-objcopy -O binary $< $@

%.elf: %.s
	riscv64-elf-gcc -Wl,-Ttext=0x0 -march=rv64g -nostdlib -o $@ $<