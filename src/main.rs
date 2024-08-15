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

enum TRAP_CODES
{
    TRAP_GETC = 0x20,  /* get character from keyboard, not echoed onto the terminal */
    TRAP_OUT = 0x21,   /* output a character */
    TRAP_PUTS = 0x22,  /* output a word string */
    TRAP_IN = 0x23,    /* get character from keyboard, echoed onto the terminal */
    TRAP_PUTSP = 0x24, /* output a byte string */
    TRAP_HALT = 0x25   /* halt the program */
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
            OP_ADD => 
            let r0: u16 = (instr >> 9) & 0x7;
            let r1: u16 = (instr >> 6) & 0x7;
            let imm_flag: u16 = (instr >> 5) & 0x1;
            if(imm_flag)
            {
                let imm5: u16 = sign_extend(instr & 0x1F, 5);
                reg[r0] = reg[r1] + imm5;
            }
            
            else
            {
                let r2: u16 = instr & 0x7;
                reg[r0] = reg[r1] + reg[r2];
            }
            update_flags(r0);
            ,
            OP_AND =>
            let r0: u16 = (instr >> 9) & 0x7;
            let r1: u16 = (instr >> 6) & 0x7;
            let imm_flag: u16 = (instr >> 5) & 0x1;
            if(imm_flag)
            {
                let imm5: u16 = sign_extend(instr & 0x1F, 5);
                reg[r0] = reg[r1] & imm5;
            }
            else
            {
                let r2: u16 = instr & 0x7;
                reg[r0] = reg[r1] & reg[r2];
            }
            update_flags(r0);
            ,
            OP_NOT =>
               let r0: u16 = (instr >> 9) & 0x7;
               let sr: u16 = (instr >> 6) & 0x7;
               reg
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

            let r0: u16 = (instr >>  9) & 0x7;
            let pc_offset: u16 = sign_extend(instr & 0x1FF,9);
            reg[r0] = mem_read(reg[R_PC] + pc_offset);
            update_flags(r0);
            ,
            OP_LDI =>
               let r0: u16 = (instr >>  9) & 0x7;
               let pc_offset: u16 = sign_extend(instr & 0x1FF,9);
               reg[r0] = mem_read(mem_read(reg[R_PC] + pc_offset));
               update_flags(r0);
            ,
            OP_LDR =>
            let r0: u16 = (instr >>  9) & 0x7;
            let offset6: u16 = sign_extend(instr & 0x1FF,6);
            let base_r: u16 = (instr >> 6) & 0x7;
            reg[r0] = mem_read(base_r + offset6);
            update_flags(r0);
            ,
            OP_LEA =>
            let r0: u16 = (instr >>  9) & 0x7;
            let pc_offset: u16 = sign_extend(instr & 0x1FF,9);
            reg[r0] = reg[R_PC] + pc_offset;
            update_flags(r0);
            ,
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
 
u16 fn sign_extend(u16 x, u16 bit_count)
{
    if((x >> (bit_count - 1)))
    {
        x |= (0xFFFF << bit_count);
    }
    return;
}

fn update_flags(u16 r)
{
    if (reg[r] == 0)
    {
        reg[R_COND] = FL_ZRO;
    }
    else if (reg[r] >> 15) /* a 1 in the left-most bit indicates negative */
    {
        reg[R_COND] = FL_NEG;
    }
    else
    {
        reg[R_COND] = FL_POS;
    }
}