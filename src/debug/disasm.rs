fn get_reg_name(reg: u32) -> String {
    let abi = [
            "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0",
            "a1", "a2", "a3", "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5",
            "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6",
        ];

    abi[reg as usize].to_string()
}

pub fn disasm_general(instr: u32) -> String {
    let opcode = instr & 0x7F;
    let funct3 = (instr >> 12) & 0x7;
    let rd = (instr >> 7) & 0x1F;
    let rs1 = (instr >> 15) & 0x1F;

    match opcode {
        0x03 => {
            // RV32/64I load functions
            let imm = (instr as i32 as i64) >> 20;

            match funct3 {
                // LB
                0x0 => { format!("lb {}, {}({})", get_reg_name(rs1), imm, get_reg_name(rs1)) }
                // LH
                0x1 => { format!("lh {}, {}({})", get_reg_name(rs1), imm, get_reg_name(rs1)) }
                // LW
                0x2 => { format!("lw {}, {}({})", get_reg_name(rs1), imm, get_reg_name(rs1)) }
                // LD
                0x3 => { format!("ld {}, {}({})", get_reg_name(rs1), imm, get_reg_name(rs1)) }
                // LBU
                0x4 => { format!("lbu {}, {}({})", get_reg_name(rs1), imm, get_reg_name(rs1)) }
                // LHU
                0x5 => { format!("lhu {}, {}({})", get_reg_name(rs1), imm, get_reg_name(rs1)) }
                // LWU
                0x6 => { format!("lwu {}, {}({})", get_reg_name(rs1), imm, get_reg_name(rs1)) }
                _ => panic!("Can't disassemble instr {:08x}", instr)
            }
        }
        0x13 => {
            // Immediate functions
            let imm = (instr as i32 as i64) >> 20;
            let funct7 = (instr >> 30) & 3;
            let shamt = (instr >> 20) & 0x3F;

            match (funct3, funct7) {
                // ADDI
                (0x0, _) => { format!("addi {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), imm) }
                // SLLI
                (0x1, _) => { format!("slli {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), shamt) }
                // SLTI
                (0x2, _) => { format!("slti {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), imm) }
                // SLTIU
                (0x3, _) => { format!("stliu {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), imm) },
                // XORI
                (0x4, _) => { format!("xori {}, {}, {:03x}", get_reg_name(rd), get_reg_name(rs1), imm) }
                // SRLI
                (0x5, 0x0) => { format!("srli {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), shamt) }
                // SRAI
                (0x5, 0x1) => { format!("srai {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), shamt) }
                // ORI
                (0x6, _) => { format!("ori {}, {}, {:03x}", get_reg_name(rd), get_reg_name(rs1), imm) }
                // ANDI
                (0x7, _) => { format!("andi {}, {}, {:03x}", get_reg_name(rd), get_reg_name(rs1), imm) }
                _ => panic!("Can't disassemble instr {:08x}", instr)
            }
        },
        0x17 => {
            // AUIPC
            let imm = (instr & 0xFFFFF000) as i32 as i64 as u64;

            format!("auipc {:08x}", imm)
        }
        0x1B => {
            // RV64I immediates
            let imm = instr as i32 as i64 >> 20;
            let funct7 = (instr >> 30) & 3;
            let shamt = (instr >> 20) & 0x1F;

            match (funct3, funct7) {
                // ADDIW
                (0x0, _) => { format!("addiw {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), imm) }
                // SLLIW
                (0x1, _) => { format!("slliw {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), shamt) }
                // SRLIW
                (0x5, 0x0) => { format!("srliw {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), shamt) }
                // SRAIW
                (0x5, 0x1) => { format!("sraiw {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), shamt) }
                _ => panic!("Can't disassemble instr {:08x}", instr)
            }
        },
        0x23 => {
            // RV32/64I store instructions
            let imm = ((((instr & 0xfe000000) as i32 as i64) >> 20) ) | ((instr >> 7) & 0x1F) as i64;
            let rs2 = (instr >> 20) & 0x1F;

            match funct3 {
                // SB
                0x0 => { format!("sb {}, {}({})", get_reg_name(rs2), imm, get_reg_name(rs1)) }
                // SH
                0x1 => { format!("sh {}, {}({})", get_reg_name(rs2), imm, get_reg_name(rs1)) }
                // SW
                0x2 => { format!("sw {}, {}({})", get_reg_name(rs2), imm, get_reg_name(rs1)) }
                // SD
                0x3 => { format!("sd {}, {}({})", get_reg_name(rs2), imm, get_reg_name(rs1)) }
                _ => panic!("Can't disassemble instr {:08x}", instr)
            }
        }
        0x37 => {
            // LUI
            let imm = (instr & 0xFFFFF000) as i32 as i64 as u64;
            
            format!("lui {}, {:016x}", get_reg_name(rd), imm)
        }
        0x33 => {
            // RV32/64I Register-to-register functions
            let rs2 = (instr >> 20) & 0x1F;
            let funct7 = (instr >> 30) & 3;

            match (funct3, funct7) {
                // ADD
                (0x0, 0x0) => { format!("add {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), get_reg_name(rs2)) }
                // SUB
                (0x0, 0x1) => { format!("sub {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), get_reg_name(rs2)) }
                // SLL
                (0x1, _) => { format!("sll {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), get_reg_name(rs2)) }
                // SLT
                (0x2, _) => { format!("slt {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), get_reg_name(rs2))}
                // SLTU
                (0x3, _) => { format!("sltu {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), get_reg_name(rs2)) }
                // XOR
                (0x4, _) => { format!("xor {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), get_reg_name(rs2)) }
                // SRL
                (0x5, 0x0) => { format!("srl {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), get_reg_name(rs2)) }
                // SRA
                (0x5, 0x1) => { format!("sra {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), get_reg_name(rs2)) }
                // OR
                (0x6, _) => { format!("or {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), get_reg_name(rs2)) }
                // AND
                (0x7, _) => { format!("and {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), get_reg_name(rs2)) }
                _ => panic!("Can't disassemble instr {:08x}", instr)
            }
        },
        0x3B => {
            // RV64I register-to-register
            let rs2 = (instr >> 20) & 0x1f;
            let funct7 = (instr >> 30) & 3;

            match (funct3, funct7) {
                // ADDW
                (0x0, 0x0) => { format!("addw {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), get_reg_name(rs2)) }
                // SUBW
                (0x0, 0x1) => { format!("subw {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), get_reg_name(rs2)) }
                // SLLW
                (0x1, _) => { format!("sllw {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), get_reg_name(rs2)) }
                // SRLW
                (0x5, 0x0) => { format!("srlw {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), get_reg_name(rs2)) }
                // SRAW
                (0x5, 0x1) => { format!("sraw {}, {}, {}", get_reg_name(rd), get_reg_name(rs1), get_reg_name(rs2)) }
                _ => panic!("Can't disassemble instr {:08x}", instr)
            }
        },
        0x63 => {
            // RV32/64I branch instructions
            let offset = (((instr & 0x80000000) as i32 as i64) >> 19) |
                ((instr & 0x80) << 4) as i64 |
                ((instr >> 20) & 0x7e0) as i64 |
                ((instr >> 7) & 0x1e) as i64;
            let rs2 = (instr >> 20) & 0x1f;

            match funct3 {
                // BEQ
                0x0 => { format!("beq {}, {}, {}", get_reg_name(rs1), get_reg_name(rs2), offset) }
                // BNE
                0x1 => { format!("bne {}, {}, {}", get_reg_name(rs1), get_reg_name(rs2), offset) }
                // BLT
                0x4 => { format!("blt {}, {}, {}", get_reg_name(rs1), get_reg_name(rs2), offset) }
                // BGE
                0x5 => { format!("bge {}, {}, {}", get_reg_name(rs1), get_reg_name(rs2), offset) }
                // BLTU 
                0x6 => { format!("bltu {}, {}, {}", get_reg_name(rs1), get_reg_name(rs2), offset) }
                // BGEU
                0x7 => { format!("bgeu {}, {}, {}", get_reg_name(rs1), get_reg_name(rs2), offset) }
                _ => panic!("Can't disassemble instr {:08x}", instr)
            }
        }
        0x67 => {
            // JALR
            let offset = (instr as i32) >> 20;
            format!("jalr {}, {}({})", get_reg_name(rd), offset, get_reg_name(rs1))
        },
        0x6F => {
                // JAL
                // Ok so getting the offset is complicated………
                let offset = (instr & 0x80000000) as i32 >> 20 | // This is the bit sign 
                (instr & 0xff000) as i32 |
                ((instr >> 9) & 0x800) as i32 |
                ((instr >> 20) & 0x7fe) as i32;

                format!("jal {}, {}", get_reg_name(rd), offset)
        },
        _ => panic!("Can't disassemble instr {:08x}", instr)
    }
}