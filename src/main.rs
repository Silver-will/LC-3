use std::io;
use std::env;

const MEMORY_MAX: usize = 1 << 16;

enum Register{
    R_R0 = 0,
    R_R1,
    R_R2,
    R_R3,
    R_R4,
    R_R5,
    R_R6,
    R_R7,
    R_PC, 
    R_COND,
    R_COUNT
};

enum Operations
{
    OP_BR = 0, /* branch */
    OP_ADD,    /* add  */
    OP_LD,     /* load */
    OP_ST,     /* store */
    OP_JSR,    /* jump register */
    OP_AND,    /* bitwise and */
    OP_LDR,    /* load register */
    OP_STR,    /* store register */
    OP_RTI,    /* unused */
    OP_NOT,    /* bitwise not */
    OP_LDI,    /* load indirect */
    OP_STI,    /* store indirect */
    OP_JMP,    /* jump */
    OP_RES,    /* reserved (unused) */
    OP_LEA,    /* load effective address */
    OP_TRAP    /* execute trap */
};

enum
{
    FL_POS = 1 << 0, /* P */
    FL_ZRO = 1 << 1, /* Z */
    FL_NEG = 1 << 2, /* N */
};

fn main() {

    let mut vm_image_path = String::new();
    io::stdin().read_line(&mut vm_image_path).
    expect("Failed to read line");

    let memory: [u16; MEMORY_MAX];

    let reg: [ui6; R_COUNT];

    reg[R_COND] = FL_ZRO;

    enum { PC_START = 0x3000 };
    reg[R_PC] = PC_START;

    let mut running = 1;
    while running != 0 {
        let instr: u16 = mem_read(reg[R_PC]++)
        let op: u16 = instr >> 12;

        match op
        {
            OP_ADD => ,
            OP_AND =>
                @{AND}
                break;
            OP_NOT =>
                @{NOT}
                break;
            OP_BR =>
                @{BR}
                break;
            OP_JMP =>
                @{JMP}
                break;
            OP_JSR =>
                @{JSR}
                break;
            OP_LD =>
                @{LD}
                break;
            OP_LDI =>
                @{LDI}
                break;
            OP_LDR =>
                @{LDR}
                break;
            OP_LEA =>
                @{LEA}
                break;
            OP_ST =>
                @{ST}
                break;
            OP_STI =>
            OP_STR =>
            OP_TRAP =>       
            OP_RES =>
            OP_RTI =>
            _ =>
        }
    }
    println!("{}",MEMORY_MAX);
}
