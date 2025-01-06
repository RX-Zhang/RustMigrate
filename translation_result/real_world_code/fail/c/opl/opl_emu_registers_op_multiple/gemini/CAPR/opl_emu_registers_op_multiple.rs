
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let start = start.wrapping_rem(32) as u32;
    let length = length.wrapping_rem(33) as u32;
    let mask = u32::wrapping_sub(1u32.wrapping_shl(length), 1);
    value.wrapping_shr(start) & mask
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    let index = (offset.wrapping_add(extra_offset) as usize) % OPL_EMU_REGISTERS_REGISTERS;
    opl_emu_bitfield(regs.m_regdata[index] as u32, start as i32, count as i32)
}

fn opl_emu_registers_op_multiple(regs: &OplEmuRegisters, opoffs: u32) -> u32 {
    opl_emu_registers_byte(regs, 0x20, 0, 4, opoffs)
}
