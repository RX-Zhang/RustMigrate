
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
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(
    regs: &OplEmuRegisters,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    let index = offset as usize + extra_offset as usize;
    if index >= OPL_EMU_REGISTERS_REGISTERS {
        return 0;
    }
    let value = regs.m_regdata[index] as u32;
    opl_emu_bitfield(value, start, count)
}

fn opl_emu_registers_ch_feedback(
    regs: &OplEmuRegisters,
    choffs: u32,
) -> u32 {
    const OFFSET: u32 = 0xc0;
    if OFFSET + choffs >= OPL_EMU_REGISTERS_REGISTERS as u32 {
        return 0;
    }
    opl_emu_registers_byte(regs, OFFSET, 1, 3, choffs)
}
