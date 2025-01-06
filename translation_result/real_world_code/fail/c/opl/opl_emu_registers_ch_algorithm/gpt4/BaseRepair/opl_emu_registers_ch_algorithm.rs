
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

fn opl_emu_bitfield(value: u32, start: usize, length: usize) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: usize, start: usize, count: usize, extra_offset: usize) -> u32 {
    let index = offset.wrapping_add(extra_offset);
    if index < regs.m_regdata.len() {
        opl_emu_bitfield(regs.m_regdata[index] as u32, start, count)
    } else {
        0 // Handle out-of-bounds access gracefully
    }
}

fn opl_emu_registers_ch_algorithm(regs: &OplEmuRegisters, choffs: usize) -> u32 {
    let byte1 = opl_emu_registers_byte(regs, 0xc0, 0, 1, choffs);
    let byte2 = opl_emu_registers_byte(regs, 0xc3, 0, 1, choffs);
    byte1 | (8 | (byte2 << 1))
}
