
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Debug, Clone)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS]>,
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>,
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start.rem_euclid(32)) & ((1 << length.rem_euclid(32)) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    let index = (offset.wrapping_add(extra_offset)) as usize;
    if index < OPL_EMU_REGISTERS_REGISTERS {
        opl_emu_bitfield(regs.m_regdata[index] as u32, start as i32, count as i32)
    } else {
        // Return 0 if out of bounds, or handle accordingly
        0
    }
}

fn opl_emu_registers_word(regs: &OplEmuRegisters, offset1: u32, start1: u32, count1: u32, offset2: u32, start2: u32, count2: u32, extra_offset: u32) -> u32 {
    let byte1 = opl_emu_registers_byte(regs, offset1, start1, count1, extra_offset);
    let byte2 = opl_emu_registers_byte(regs, offset2, start2, count2, extra_offset);
    (byte1 << count2) | byte2
}
