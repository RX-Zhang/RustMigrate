
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

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    let shifted = value.wrapping_shr(start);
    let mask = (1u32).wrapping_shl(length).wrapping_sub(1);
    shifted & mask
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    let idx = offset.wrapping_add(extra_offset) as usize;
    if idx >= OPL_EMU_REGISTERS_REGISTERS {
        return 0; // Return 0 for out-of-bounds access
    }
    let value = regs.m_regdata[idx] as u32;
    opl_emu_bitfield(value, start, count)
}

fn opl_emu_registers_op_multiple(regs: &OplEmuRegisters, opoffs: u32) -> u32 {
    opl_emu_registers_byte(regs, 0x20, 0, 4, opoffs)
}
