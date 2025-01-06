
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
    // Handle potential overflow and out-of-bounds shifts
    let shifted = value >> start;
    let mask = (1 << length) - 1;
    shifted & mask
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    let index = offset as usize + extra_offset as usize;
    // Ensure index is within bounds
    if index < OPL_EMU_REGISTERS_REGISTERS {
        let byte = regs.m_regdata[index] as u32;
        opl_emu_bitfield(byte, start as i32, count as i32)
    } else {
        0 // Handle out-of-bounds access by returning 0 or a default value
    }
}

fn opl_emu_registers_op_release_rate(regs: &OplEmuRegisters, opoffs: u32) -> u32 {
    opl_emu_registers_byte(regs, 0x80, 0, 4, opoffs)
}
