
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitOrAssign;

const CONST_PL_EMU_REGISTERS_REG_MASK: u16 = 0x1ff;
const OPL_EMU_REGISTERS_CHANNELS: usize = 9;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 32;
const OPL_EMU_REGISTERS_OPERATOR_PER_CHANNEL: usize = 2;
const OPL_EMU_REGISTERS_REGISTERS: usize = 256;

fn opl_emu_registers_write(
regs: &mut OplEmuRegisters,
index: u16,
data: u8,
channel: &mut u32,
opmask: &mut u32,
) -> bool {
let reg_index = index as usize & (OPL_EMU_REGISTERS_REGISTERS - 1);
match index & CONST_PL_EMU_REGISTERS_REG_MASK {
OPL_EMU_REGISTERS_REG_MODE => {
regs.mode = data & 0x0f;
true
}
(OPL_EMU_REGISTERS_WAVEFORM_LENGTH_MSB..=OPL_EMU_REGISTERS_WAVEFORM_LENGTH_LSB) => {
let channel_index = reg_index >> 1;
let shifted_data = (data as u16) << 8;
regs.waveform_length[channel_index].0 = regs.waveform_length[channel_index].0.wrapping_add(shifted_data);
regs.waveform_length[channel_index].1 = (shifted_data | (data as u16)) * 2;
true
}
OPL_EMU_REGISTERS_FEEDBACK => {
regs.feedback = data & 0x03;
true
}
OPL_EMU_REGISTERS_ALGORITHM => {
regs.algorithm = data & 0x07;
true
}
OPL_EMU_REGISTERS_OUTPUT_LEVELS => {
regs.output_levels = data & 0x0f;
true
}
(OPL_EMU_REGISTERS_MUL_MULTIPLICAND_MSB..=OPL_EMU_REGISTERS_MUL_MULTIPLIER_LSB) => {
let channel_index = reg_index >> 1;
let shifted_data = (data as u16) << 8;
regs.mul_multplicand[channel_index].0 = regs.mul_multplicand[channel_index].0.wrapping_add(shifted_data);
regs.mul_multiplier[channel_index].0 = regs.mul_multiplier[channel_index].0.wrapping_add(shifted_data);
regs.mul_multplicand[channel_index].1 = (shifted_data | (data as u16)) * 2;
regs.mul_multiplier[channel_index].1 = (shifted_data | (data as u16)) * 2;
true
}
_ => {
if index >= 0x100 && *channel < OPL_EMU_REGISTERS_CHANNELS as u32 {
let shift = ((index - 0x100) & 0x0f) as u32;
let mask = 1 << shift;
let reg_index = *channel * OPL_EMU_REGISTERS_OPERATOR_PER_CHANNEL + (index >> 4) as usize;
if index & 0x20 != 0 {
*opmask &= !(mask);
} else {
*opmask |= mask;
}
regs.operator[reg_index].enable = (*opmask & mask) != 0;
regs.operator[reg_index].key_on = (data & 0x10) != 0;
regs.operator[reg_index].block = (data >> 1) & 0x0f;
regs.operator[reg_index].feedback = (data >> 5) & 0x03;
regs.operator[reg_index].algorithm = data >> 7;
if data & 0x20 != 0 {
regs.operator[reg_index].enable = false;
}
true
} else {
false
}
}
}
}
