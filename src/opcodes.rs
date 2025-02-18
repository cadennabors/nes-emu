use crate::cpu::AddressingMode;

pub struct Opcode {
    pub name : u8,
    pub bytes : u8,
    pub cycles : u8,
    pub addressing_mode : AddressingMode
}

impl Opcode {
    pub fn new(name : u8, bytes : u8, cycles : u8, addressing_mode : AddressingMode) -> Self {
        Opcode {
        name,
        bytes,
        cycles,
        addressing_mode
     }
    }
}
// Explanations taken from https://www.nesdev.org/wiki/Instruction_reference

// ACCESS 
// LDA loads a memory value into the accumulator.
pub const LDA_IMM : u8 = 0xA9;
pub const LDA_ZP : u8 = 0xA5;
pub const LDA_ZP_X : u8 = 0xB5;
pub const LDA_ABS : u8 = 0xAD;
pub const LDA_ABS_X : u8 = 0xBD;
pub const LDA_ABS_Y : u8 = 0xB9;
pub const LDA_IND_X : u8 = 0xA1;
pub const LDA_IND_Y : u8 = 0xB1;

// STA stores the accumulator value into memory.
pub const STA_ZP : u8 = 0x85;
pub const STA_ZP_X : u8 = 0x95;
pub const STA_ABS : u8 = 0x8D;
pub const STA_ABS_X : u8 = 0x9D;
pub const STA_ABS_Y : u8 = 0x99;
pub const STA_IND_X : u8 = 0x81;
pub const STA_ZP_IND_Y : u8 = 0x91;

// LDX loads a memory value into the X register.
pub const LDX_IMM : u8 = 0xA2;
pub const LDX_ZP : u8 = 0xA6;
pub const LDX_ZP_Y : u8 = 0xB6;
pub const LDX_ABS : u8 = 0xAE;
pub const LDX_ABS_Y : u8 = 0xBE;

// STX stores the X register value into memory.
pub const STX_ZP : u8 = 0x86;
pub const STX_ZP_Y : u8 = 0x96;
pub const STX_ABS : u8 = 0x8E;

// LDX loads a memory value into the Y register.
pub const LDY_IMM : u8 = 0xA0;
pub const LDY_ZP : u8 = 0xA4;
pub const LDY_ZP_X : u8 = 0xB4;
pub const LDY_ABS : u8 = 0xAC;
pub const LDY_ABS_X : u8 = 0xBC;

// STY stores the Y register value into memory.
pub const STY_ZP : u8 = 0x84;
pub const STY_ZP_X : u8 = 0x94;
pub const STY_ABS : u8 = 0x8C;

// TRANSFER
// TAX copies the accumulator value to the X register.
pub const TAX : u8 = 0xAA;

// TXA copies the X register value to the accumulator.
pub const TXA : u8 = 0x8A;

// TAY copies the accumulator value to the Y register.
pub const TAY : u8 = 0xA8;

// TYA copies the Y register value to the accumulator.
pub const TYA : u8 = 0x98;

// ARITHMETIC
/* ADC adds the carry flag and a memory value to the accumulator. 
The carry flag is then set to the carry value coming out of bit 7,
allowing values larger than 1 byte to be added together by carrying 
the 1 into the next byte's addition. This can also be thought of as
unsigned overflow. It is common to clear carry with CLC before adding
the first byte to ensure it is in a known state, avoiding an off-by-one
error. The overflow flag indicates whether signed overflow or underflow
occurred. This happens if both inputs are positive and the result is
negative, or both are negative and the result is positive. */
pub const ADC_IMM : u8 = 0x69;
pub const ADC_ZP : u8 = 0x65;
pub const ADC_ZP_X : u8 = 0x75;
pub const ADC_ABS : u8 = 0x6D;
pub const ADC_ABS_X : u8 = 0x7D;
pub const ADC_ABS_Y : u8 = 0x79;
pub const ADC_IND_X : u8 = 0x61;
pub const ADC_IND_Y : u8 = 0x71;

/* SBC subtracts a memory value and the bitwise NOT of carry from the
accumulator. It does this by adding the bitwise NOT of the memory value 
using ADC. This implementation detail explains the backward nature of 
carry; SBC subtracts 1 more when carry is clear, not when it's set, and
carry is cleared when it underflows and set otherwise. 
As with ADC, carry allows the borrow from one subtraction to be carried
into the next subtraction, allowing subtraction of values larger than 1
byte. It is common to set carry with SEC before subtracting the first
byte to ensure it is in a known state, avoiding an off-by-one error.

Overflow works the same as with ADC, except with an inverted memory value. 
Therefore, overflow or underflow occur if the result's sign is different 
from A's and the same as the memory value's. */
pub const SBC_IMM : u8 = 0xE9;
pub const SBC_ZP : u8 = 0xE5;
pub const SBC_ZP_X : u8 = 0xF5;
pub const SBC_ABS : u8 = 0xED;
pub const SBC_ABS_X : u8 = 0xFD;
pub const SBC_ABS_Y : u8 = 0xF9;
pub const SBC_IND_X : u8 = 0xE1;
pub const SBC_IND_Y : u8 = 0xF1;

/* INC adds 1 to a memory location. Notably, there is no version of this
instruction for the accumulator; ADC or SBC must be used, instead.

This is a read-modify-write instruction, meaning that it first writes
the original value back to memory before the modified value. 
This extra write can matter if targeting a hardware register.

Note that increment does not affect carry nor overflow. */
pub const INC_ZP : u8 = 0xE6;
pub const INC_ZP_X : u8 = 0xF6;
pub const INC_ABS : u8 = 0xEE;
pub const INC_ABS_X : u8 = 0xFE;

/* DEC subtracts 1 from a memory location. Notably, there is no version 
of this instruction for the accumulator; ADC or SBC must be used, instead.

This is a read-modify-write instruction, meaning that it first writes 
the original value back to memory before the modified value. 
This extra write can matter if targeting a hardware register.

Note that decrement does not affect carry nor overflow. */
pub const DEC_ZP : u8 = 0xC6;
pub const DEC_ZP_X : u8 = 0xD6;
pub const DEC_ABS : u8 = 0xCE;
pub const DEC_ABS_X : u8 = 0xDE;

// INX adds 1 to the X register. Note that it does not affect carry nor overflow.
pub const INX : u8 = 0xE8;

// DEX subtracts 1 from the X register. Note that it does not affect carry nor overflow.
pub const DEX : u8 = 0xCA;

// INY adds 1 to the Y register. Note that it does not affect carry nor overflow.
pub const INY : u8 = 0xC8;

// DEY subtracts 1 from the Y register. Note that it does not affect carry nor overflow.
pub const DEY : u8 = 0x88;

// SHIFT
/* ASL shifts all of the bits of a memory value or the accumulator one 
position to the left, moving the value of each bit into the next bit. 
Bit 7 is shifted into the carry flag, and 0 is shifted into bit 0. 
This is equivalent to multiplying an unsigned value by 2, with carry
indicating overflow.

This is a read-modify-write instruction, meaning that its addressing 
modes that operate on memory first write the original value back to 
memory before the modified value. This extra write can matter if 
targeting a hardware register. */
pub const ASL_ACC : u8 = 0x0A;
pub const ASL_ZP : u8 = 0x06;
pub const ASL_ZP_X : u8 = 0x16;
pub const ASL_ABS : u8 = 0x0E;
pub const ASL_ABS_X : u8 = 0x1E;

/* LSR shifts all of the bits of a memory value or the accumulator one 
position to the right, moving the value of each bit into the next bit.
0 is shifted into bit 7, and bit 0 is shifted into the carry flag. 
This is equivalent to dividing an unsigned value by 2 and rounding down, 
with the remainder in carry.

This is a read-modify-write instruction, meaning that its addressing 
modes that operate on memory first write the original value back to 
memory before the modified value. This extra write can matter if 
targeting a hardware register. */
pub const LSR_ACC : u8 = 0x4A;
pub const LSR_ZP : u8 = 0x46;
pub const LSR_ZP_X : u8 = 0x56;
pub const LSR_ABS : u8 = 0x4E;
pub const LSR_ABS_X : u8 = 0x5E;

/* ROL shifts a memory value or the accumulator to the left, moving the 
value of each bit into the next bit and treating the carry flag as 
though it is both above bit 7 and below bit 0. Specifically, the value 
in carry is shifted into bit 0, and bit 7 is shifted into carry. 
Rotating left 9 times simply returns the value and carry back to their 
original state.

This is a read-modify-write instruction, meaning that its addressing 
modes that operate on memory first write the original value back to 
memory before the modified value. This extra write can matter if 
targeting a hardware register. */
pub const ROL_ACC : u8 = 0x2A;
pub const ROL_ZP : u8 = 0x26;
pub const ROL_ZP_X : u8 = 0x36;
pub const ROL_ABS : u8 = 0x2E;
pub const ROL_ABS_X : u8 = 0x3E;

/* ROR shifts a memory value or the accumulator to the right, moving the 
value of each bit into the next bit and treating the carry flag as 
though it is both above bit 7 and below bit 0. Specifically, the value 
in carry is shifted into bit 7, and bit 0 is shifted into carry. 
Rotating right 9 times simply returns the value and carry back to their 
original state.

This is a read-modify-write instruction, meaning that its addressing 
modes that operate on memory first write the original value back to 
memory before the modified value. This extra write can matter if 
targeting a hardware register.*/
pub const ROR_ACC : u8 = 0x6A;
pub const ROR_ZP : u8 = 0x66;
pub const ROR_ZP_X : u8 = 0x76;
pub const ROR_ABS : u8 = 0x6E;
pub const ROR_ABS_X : u8 = 0x7E;

// BITWISE
/* This ANDs a memory value and the accumulator, bit by bit. If both 
input bits are 1, the resulting bit is 1. Otherwise, it is 0.*/
pub const AND_IMM : u8 = 0x29;
pub const AND_ZP : u8 = 0x25;
pub const AND_ZP_X : u8 = 0x35;
pub const AND_ABS : u8 = 0x2D;
pub const AND_ABS_X : u8 = 0x3D;
pub const AND_ABS_Y : u8 = 0x39;
pub const AND_IND_X : u8 = 0x21;
pub const AND_IND_Y : u8 = 0x31;

/* ORA inclusive-ORs a memory value and the accumulator, bit by bit. 
If either input bit is 1, the resulting bit is 1. Otherwise, it is 0. */
pub const ORA_IMM : u8 = 0x09;
pub const ORA_ZP : u8 = 0x05;
pub const ORA_ZP_X : u8 = 0x15;
pub const ORA_ABS : u8 = 0x0D;
pub const ORA_ABS_X : u8 = 0x1D;
pub const ORA_ABS_Y : u8 = 0x19;
pub const ORA_IND_X : u8 = 0x01;
pub const ORA_IND_Y : u8 = 0x11;

/* EOR exclusive-ORs a memory value and the accumulator, bit by bit. If the input bits are different, the resulting bit is 1. If they are the same, it is 0. This operation is also known as XOR.

6502 doesn't have bitwise NOT instruction, but using EOR with value 
$FF has the same behavior, inverting every bit of the other value. 
In fact, EOR can be thought of as NOT with a bitmask; all of the 1 
bits in one value have the effect of inverting the corresponding bit
in the other value, while 0 bits do nothing. */
pub const EOR_IMM : u8 = 0x49;
pub const EOR_ZP : u8 = 0x45;
pub const EOR_ZP_X : u8 = 0x55;
pub const EOR_ABS : u8 = 0x4D;
pub const EOR_ABS_X : u8 = 0x5D;
pub const EOR_ABS_Y : u8 = 0x59;
pub const EOR_IND_X : u8 = 0x41;
pub const EOR_IND_Y : u8 = 0x51;

/* BIT modifies flags, but does not change memory or registers. 
The zero flag is set depending on the result of the accumulator AND 
memory value, effectively applying a bitmask and then checking if any 
bits are set. Bits 7 and 6 of the memory value are loaded directly into
the negative and overflow flags, allowing them to be easily checked
without having to load a mask into A.

Because BIT only changes CPU flags, it is sometimes used to 
trigger the read side effects of a hardware register without 
clobbering any CPU registers, or even to waste cycles as a 
3-cycle NOP. As an advanced trick, it is occasionally used to hide a 
1- or 2-byte instruction in its operand that is only executed if jumped 
to directly, allowing two code paths to be interleaved. However, 
because the instruction in the operand is treated as an address from 
which to read, this carries risk of triggering side effects if it reads 
a hardware register. This trick can be useful when working under tight 
constraints on space, time, or register usage. */
pub const BIT_ZP : u8 = 0x24;
pub const BIT_ABS : u8 = 0x2C;

// COMPARE