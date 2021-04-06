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
                0x0 => { format!("lb x{:02}, {}(x{:02})", rs1, imm, rs1) }
                // LH
                0x1 => { format!("lh x{:02}, {}(x{:02})", rs1, imm, rs1) }
                // LW
                0x2 => { format!("lw x{:02}, {}(x{:02})", rs1, imm, rs1) }
                // LD
                0x3 => { format!("ld x{:02}, {}(x{:02})", rs1, imm, rs1) }
                // LBU
                0x4 => { format!("lbu x{:02}, {}(x{:02})", rs1, imm, rs1) }
                // LHU
                0x5 => { format!("lhu x{:02}, {}(x{:02})", rs1, imm, rs1) }
                // LWU
                0x6 => { format!("lwu x{:02}, {}(x{:02})", rs1, imm, rs1) }
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
                (0x0, _) => { format!("addi x{:02}, x{:02}, {:03x}", rd, rs1, imm) }
                // SLLI
                (0x1, _) => { format!("slli x{:02}, x{:02}, {:03x}", rd, rs1, shamt) }
                // SLTI
                (0x2, _) => { format!("slti x{:02}, x{:02}, {:03x}", rd, rs1, imm as i64) }
                // SLTIU
                (0x3, _) => { format!("stliu x{:02}, x{:02}, {:03x}", rd, rs1, imm) },
                // XORI
                (0x4, _) => { format!("xori x{:02}, x{:02}, {:03x}", rd, rs1, imm) }
                // SRLI
                (0x5, 0x0) => { format!("srli x{:02}, x{:02}, {:03x}", rd, rs1, shamt) }
                // SRAI
                (0x5, 0x1) => { format!("srai x{:02}, x{:02}, {:03x}", rd, rs1, shamt) }
                // ORI
                (0x6, _) => { format!("ori x{:02}, x{:02}, {:03x}", rd, rs1, imm) }
                // ANDI
                (0x7, _) => { format!("andi x{:02}, x{:02}, {:03x}", rd, rs1, imm) }
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
                (0x0, _) => { format!("addiw x{:02}, x{:02}, {}", rd, rs1, imm) }
                // SLLIW
                (0x1, _) => { format!("slliw x{:02}, x{:02}, {}", rd, rs1, shamt) }
                // SRLIW
                (0x5, 0x0) => { format!("srliw x{:02}, x{:02}, {}", rd, rs1, shamt) }
                // SRAIW
                (0x5, 0x1) => { format!("sraiw x{:02}, x{:02}, {}", rd, rs1, shamt) }
                _ => panic!("Can't disassemble instr {:08x}", instr)
            }
        },
        0x23 => {
            // RV32/64I store instructions
            let imm = ((((instr & 0xfe000000) as i32 as i64) >> 20) ) | ((instr >> 7) & 0x1F) as i64;
            let rs2 = (instr >> 20) & 0x1F;

            match funct3 {
                // SB
                0x0 => { format!("sb x{:02}, {}(x{:02})", rs2, imm, rs1) }
                // SH
                0x1 => { format!("sh x{:02}, {}(x{:02})", rs2, imm, rs1) }
                // SW
                0x2 => { format!("sw x{:02}, {}(x{:02})", rs2, imm, rs1) }
                // SD
                0x3 => { format!("sd x{:02}, {}(x{:02})", rs2, imm, rs1) }
                _ => panic!("Can't disassemble instr {:08x}", instr)
            }
        }
        0x37 => {
            // LUI
            let imm = (instr & 0xFFFFF000) as i32 as i64 as u64;
            
            format!("lui x{:02}, {:016x}", rd, imm)
        }
        0x33 => {
            // RV32/64I Register-to-register functions
            let rs2 = (instr >> 20) & 0x1F;
            let funct7 = (instr >> 30) & 3;

            match (funct3, funct7) {
                // ADD
                (0x0, 0x0) => { format!("add x{:02}, x{:02}, x{:02}", rd, rs1, rs2) }
                // SUB
                (0x0, 0x1) => { format!("sub x{:02}, x{:02}, x{:02}", rd, rs1, rs2) }
                // SLL
                (0x1, _) => { format!("sll x{:02}, x{:02}, x{:02}", rd, rs1, rs2) }
                // SLT
                (0x2, _) => { format!("slt x{:02}, x{:02}, x{:02}", rd, rs1, rs2)}
                // SLTU
                (0x3, _) => { format!("sltu x{:02}, x{:02}, x{:02}", rd, rs1, rs2) }
                // XOR
                (0x4, _) => { format!("xor x{:02}, x{:02}, x{:02}", rd, rs1, rs2) }
                // SRL
                (0x5, 0x0) => { format!("srl x{:02}, x{:02}, x{:02}", rd, rs1, rs2) }
                // SRA
                (0x5, 0x1) => { format!("sra x{:02}, x{:02}, x{:02}", rd, rs1, rs2) }
                // OR
                (0x6, _) => { format!("or x{:02}, x{:02}, x{:02}", rd, rs1, rs2) }
                // AND
                (0x7, _) => { format!("and x{:02}, x{:02}, x{:02}", rd, rs1, rs2) }
                _ => panic!("Can't disassemble instr {:08x}", instr)
            }
        },
        0x3B => {
            // RV64I register-to-register
            let rs2 = (instr >> 20) & 0x1f;
            let funct7 = (instr >> 30) & 3;

            match (funct3, funct7) {
                // ADDW
                (0x0, 0x0) => { format!("addw x{:02}, x{:02}, x{:02}", rd, rs1, rs2) }
                // SUBW
                (0x0, 0x1) => { format!("subw x{:02}, x{:02}, x{:02}", rd, rs1, rs2) }
                // SLLW
                (0x1, _) => { format!("sllw x{:02}, x{:02}, x{:02}", rd, rs1, rs2) }
                // SRLW
                (0x5, 0x0) => { format!("srlw x{:02}, x{:02}, x{:02}", rd, rs1, rs2) }
                // SRAW
                (0x5, 0x1) => { format!("sraw x{:02}, x{:02}, x{:02}", rd, rs1, rs2) }
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
                0x0 => { format!("beq x{:02}, x{:02}, {}", rs1, rs2, offset) }
                // BNE
                0x1 => { format!("bne x{:02}, x{:02}, {}", rs1, rs2, offset) }
                // BLT
                0x4 => { format!("blt x{:02}, x{:02}, {}", rs1, rs2, offset) }
                // BGE
                0x5 => { format!("bge x{:02}, x{:02}, {}", rs1, rs2, offset) }
                // BLTU 
                0x6 => { format!("bltu x{:02}, x{:02}, {}", rs1, rs2, offset) }
                // BGEU
                0x7 => { format!("bgeu x{:02}, x{:02}, {}", rs1, rs2, offset) }
                _ => panic!("Can't disassemble instr {:08x}", instr)
            }
        }
        0x67 => {
            // JALR
            let offset = (instr as i32) >> 20;
            format!("jalr x{:02}, {}(x{:02})", rd, offset, rs1)
        },
        0x6F => {
                // JAL
                // Ok so getting the offset is complicated………
                let offset = (instr & 0x80000000) as i32 >> 20 | // This is the bit sign 
                (instr & 0xff000) as i32 |
                ((instr >> 9) & 0x800) as i32 |
                ((instr >> 20) & 0x7fe) as i32;

                format!("jal x{:02}, {}", rd, offset)
        },
        _ => panic!("Can't disassemble instr {:08x}", instr)
    }
}