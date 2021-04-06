%.bin: %.elf
	riscv64-elf-objcopy -O binary $< $@

%.elf: %.s
	riscv64-elf-gcc -Wl,-Ttext=0x0 -march=rv64g -nostdlib -o $@ $<

%.elf: %.o
	riscv64-elf-ld -T riscv64.ld -o $@ crt0.o $<
	rm crt0.o

%.o: %.c
	riscv64-elf-gcc  -nostdlib -march=rv64g -c crt0.s $<