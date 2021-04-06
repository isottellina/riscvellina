use crate::bus::{BusSize, Bus};
use std::io::Read;

#[derive(Default)]
pub struct CPU {
    pc: u64,
    regs: [u64; 32],
    bus: Bus,
    pub halt: bool
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0x80000000,
            regs: [0; 32],
            bus: Bus::new(0x1000000),
            halt: false
        }
    }

    pub fn load_code(&mut self, mut file: std::fs::File) -> std::io::Result<()> {
        let mut code = vec![];
        file.read_to_end(&mut code)?;
        self.bus.load_code(code);

        Ok(())
    }

    fn fetch(&mut self) -> u32 {
        let instr = self.bus.load32(self.pc);
        self.pc += 4;
        println!("{:08x}", instr);

        instr
    }

    fn execute(&mut self, instr: u32) {
        // TODO: Meilleur technique pour arrêter le processeur mdr
        if instr == 0x00000013 { self.halt = true; return; }

        let opcode = instr & 0x7F;
        let funct3 = (instr >> 12) & 0x7;
        let rd = (instr >> 7) & 0x1F;
        let rs1 = (instr >> 15) & 0x1F;

        match opcode {
            0x03 => {
                // RV32/64I load functions
                let imm = ((instr as i32 as i64) >> 20) as u64;
                let addr = self.read_reg(rs1).wrapping_add(imm);

                match funct3 {
                    // LB
                    0x0 => { let value = self.bus.load8(addr) as i8 as i64 as u64; self.write_reg(rd, value) }
                    // LH
                    0x1 => { let value = self.bus.load16(addr) as i16 as i64 as u64; self.write_reg(rd, value) }
                    // LW
                    0x2 => { let value = self.bus.load32(addr) as i32 as i64 as u64; self.write_reg(rd, value) }
                    // LBU
                    0x4 => { let value = self.bus.load8(addr) as u64; self.write_reg(rd, value) }
                    // LHU
                    0x5 => { let value = self.bus.load16(addr) as u64; self.write_reg(rd, value) }
                    // LWU
                    0x6 => { let value = self.bus.load32(addr) as u64; self.write_reg(rd, value) }
                    // LD
                    0x7 => { let value = self.bus.load64(addr); self.write_reg(rd, value) }
                    _ => unimplemented!("funct3 not yet implemented ({:2x}, {:1x})", opcode, funct3)
                }
            },
            0x13 => {
                // Immediate functions
                let imm = ((instr as i32 as i64) >> 20) as u64;
                let funct7 = (instr >> 30) & 3;
                let shamt = (instr >> 20) & 0x3F;

                match funct3 {
                    // ADDI
                    0x0 => { self.write_reg(rd, self.read_reg(rs1).wrapping_add(imm)) }
                    // SLLI
                    0x1 => { self.write_reg(rd, self.read_reg(rs1) << shamt) }
                    // SLTI
                    0x2 => { self.write_reg(rd, ((self.read_reg(rs1) as i64) < imm as i64) as u64) }
                    // SLTIU
                    0x3 => { self.write_reg(rd, (self.read_reg(rs1) < imm) as u64)},
                    // XORI
                    0x4 => { self.write_reg(rd, self.read_reg(rs1) ^ imm)}
                    0x5 => {
                        match funct7 {
                            // SRLI
                            0x0 => { self.write_reg(rd, self.read_reg(rs1) >> shamt) }
                            // SRAI
                            0x1 => { self.write_reg(rd, (self.read_reg(rs1) as i64 >> shamt) as u64) }
                            _ => panic!("Bad instruction! {:08x}", instr)
                        }
                    }
                    // ORI
                    0x6 => { self.write_reg(rd, self.read_reg(rs1) | imm) }
                    // ANDI
                    0x7 => { self.write_reg(rd, self.read_reg(rs1) & imm) }
                    _ => unimplemented!("funct3 not implemented! ({:01x})", funct3)
                }
            },
            0x17 => {
                // AUIPC
                let imm = (instr & 0xFFFFF000) as i32 as i64 as u64;
                let value = self.pc.wrapping_add(imm).wrapping_sub(4);

                self.write_reg(rd, value);
            }
            0x1B => {
                // RV64I immediates
                let imm = (instr as i32 as i64 >> 20) as u64;
                let funct7 = (instr >> 30) & 3;
                let shamt = (instr >> 20) & 0x1F;

                match funct3 {
                    // ADDIW
                    0x0 => { self.write_reg(rd, self.read_reg(rs1).wrapping_add(imm as i64 as i32 as u64))}
                    // SLLIW
                    0x1 => { self.write_reg(rd, (self.read_reg(rs1) << shamt) as i32 as u64) }
                    0x5 => {
                        println!("{:08x} {}", self.read_reg(rs1), self.read_reg(rs1) as i32);
                        match funct7 {
                            // SRLIW
                            0x0 => { self.write_reg(rd, (self.read_reg(rs1) >> shamt) as i32 as u32 as u64) }
                            // SRAIW
                            0x1 => { self.write_reg(rd, ((self.read_reg(rs1) as i32) >> shamt) as u32 as u64) },
                            _ => panic!("Bad instruction!")
                        }
                    }
                    _ => unimplemented!("Not implemented yet")
                }
            },
            0x23 => {
                // RV32/64I store instructions
                let imm = (((instr as i64) >> 20) & 0xFE0 ) as u64 | ((instr >> 7) & 0x1F) as u64;
                let rs2 = (instr >> 20) & 0x1F;
                let addr = self.read_reg(rs1).wrapping_add(imm);
                let value = self.read_reg(rs2);

                match funct3 {
                    // SB
                    0x0 => { self.bus.store8(addr, value as u8) }
                    // SH
                    0x1 => { self.bus.store16(addr, value as u16) }
                    // SW
                    0x2 => { self.bus.store32(addr, value as u32) }
                    // SD
                    0x3 => { self.bus.store64(addr, value) }
                    _ => panic!("Bad instruction")
                }
            }
            0x37 => {
                // LUI
                let imm = (instr & 0xFFFFF000) as i32 as i64 as u64;
                self.write_reg(rd, imm as i32 as i64 as u64);
            }
            0x33 => {
                // RV32/64I Register-to-register functions
                let rs2 = (instr >> 20) & 0x1F;
                let funct7 = (instr >> 30) & 3;

                match funct3 {
                    0x0 => {
                        match funct7 {
                            // ADD
                            0x0 => { self.write_reg(rd, self.read_reg(rs1).wrapping_add(self.read_reg(rs2))) }
                            // SUB
                            0x1 => { self.write_reg(rd, self.read_reg(rs1).wrapping_sub(self.read_reg(rs2)))}
                            _ => panic!("Bad instruction!")
                        }
                    }
                    // SLL
                    0x1 => { self.write_reg(rd, self.read_reg(rs1) << (self.read_reg(rs2) & 0x3F)) }
                    // SLT
                    0x2 => { self.write_reg(rd, ((self.read_reg(rs1) as i64) < (self.read_reg(rs2) as i64)) as u64)}
                    // SLTU
                    0x3 => { self.write_reg(rd, (self.read_reg(rs1) < self.read_reg(rs2)) as u64) }
                    // XOR
                    0x4 => { self.write_reg(rd, self.read_reg(rs1) ^ self.read_reg(rs2)) }
                    0x5 => {
                        match funct7 {
                            // SRL
                            0x0 => { self.write_reg(rd, self.read_reg(rs1) >> (self.read_reg(rs2) & 0x3F)) }
                            // SRA
                            0x1 => { self.write_reg(rd, ((self.read_reg(rs1) as i64) >> (self.read_reg(rs2) & 0x3F)) as u64) }
                            _ => panic!("Bad instruction!")
                        }
                    }
                    // OR
                    0x6 => { self.write_reg(rd, self.read_reg(rs1) | self.read_reg(rs2)) }
                    // AND
                    0x7 => { self.write_reg(rd, self.read_reg(rs1) & self.read_reg(rs2)) }
                    _ => unimplemented!("funct3 not implemented!")
                }
            },
            0x3B => {
                // RV64I register-to-register
                let rs2 = (instr >> 20) & 0x1f;
                let funct7 = (instr >> 30) & 3;

                match funct3 {
                    0x0 => { 
                        match funct7 {
                            0x0 => { self.write_reg(rd, self.read_reg(rs1).wrapping_add(self.read_reg(rs2) as i64 as i32 as u64)) }
                            0x1 => { self.write_reg(rd, self.read_reg(rs1).wrapping_sub(self.read_reg(rs2) as i64 as i32 as u64)) }
                            _ => panic!("Bad instruction!")
                        }
                    }
                    // SLLW
                    0x1 => { self.write_reg(rd, (self.read_reg(rs1) << (self.read_reg(rs2) & 0x1f)) as i32 as u64) }
                    0x5 => {

                        match funct7 {
                            // SRLW
                            0x0 => { self.write_reg(rd, (self.read_reg(rs1) >> (self.read_reg(rs2) & 0x1f)) as i32 as u32 as u64) }
                            // SRAW
                            0x1 => { self.write_reg(rd, ((self.read_reg(rs1) as i32) >> (self.read_reg(rs2) & 0x1f)) as u32 as u64) },
                            _ => panic!("Bad instruction!")
                        }
                    }
                    _ => unimplemented!("Not implemented yet")
                }
            },
            0x63 => {
                // RV32/64I branch instructions
                let offset = (((instr & 0x80000000) as i32 as i64) >> 19) as u64 |
                    ((instr & 0x80) << 4) as u64 |
                    ((instr >> 20) & 0x7e0) as u64 |
                    ((instr >> 7) & 0x1e) as u64;
                let addr = self.pc.wrapping_add(offset).wrapping_sub(4);
                let rs2 = (instr >> 20) & 0x1f;

                match funct3 {
                    // BEQ
                    0x0 => { if self.read_reg(rs1) == self.read_reg(rs2) { self.pc = addr; } }
                    // BNE
                    0x1 => { if self.read_reg(rs1) != self.read_reg(rs2) { self.pc = addr; } }
                    // BLT
                    0x4 => { if (self.read_reg(rs1) as i64) < self.read_reg(rs2) as i64 { self.pc = addr; } }
                    // BGE
                    0x5 => { if self.read_reg(rs1) as i64 >= self.read_reg(rs2) as i64 { self.pc = addr; } }
                    // BLTU 
                    0x6 => { if self.read_reg(rs1) < self.read_reg(rs2) { self.pc = addr; } }
                    // BGEU
                    0x7 => { if self.read_reg(rs1) >= self.read_reg(rs2) { self.pc = addr; } }
                    _ => panic!("Bad instruction!")
                }
            }
            0x67 => {
                // JALR
                let offset = (instr as i32) >> 20;
                let old_pc = self.pc;
                let target = (self.read_reg(rs1).wrapping_add(offset as u32 as u64)) & !1;

                self.pc = target as BusSize;
                self.write_reg(rd, old_pc as u64);
            },
            0x6F => {
                // JAL
                // Ok so getting the offset is complicated………
                let offset = ((instr & 0x80000000) as i32 >> 20) as u64 | // This is the bit sign 
                (instr & 0xff000) as u64 |
                ((instr >> 9) & 0x800) as u64 |
                ((instr >> 20) & 0x1fe) as u64;

                self.write_reg(rd, self.pc);
                self.pc = self.pc.wrapping_add(offset).wrapping_sub(4);
            },
            _ => unimplemented!("Opcode not implemented! ({:02x})", opcode)
        }
    }

    pub fn run_instr(&mut self) {
        let instr = self.fetch();
        self.execute(instr);
    }

    pub fn read_reg(&self, reg: u32) -> u64 {
        if reg == 0 {
            0
        } else {
            self.regs[reg as usize]
        }
    }

    pub fn write_reg(&mut self, reg: u32, value: u64) {
        if reg != 0 {
            self.regs[reg as usize] = value
        }
    }
}

impl std::fmt::Debug for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let abi = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];

        let mut output = String::new();
        for i in (0..32).step_by(4) {
            output = format!("{}\n{}",
                output,
                format!("x{:02}({})={:016x} x{:02}({})={:016x} x{:02}({})={:016x} x{:02}({})={:016x}",
                        i, abi[i], self.read_reg(i as u32),
                        i + 1, abi[i + 1], self.read_reg(i as u32 + 1),
                        i + 2, abi[i + 2], self.read_reg(i as u32 + 2),
                        i + 3, abi[i + 3], self.read_reg(i as u32 + 3),
                )
            )
        }

        write!(f, "{}", output)
    }
}