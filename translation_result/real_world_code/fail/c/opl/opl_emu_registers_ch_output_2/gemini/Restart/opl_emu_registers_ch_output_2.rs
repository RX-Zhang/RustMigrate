

// Define the structure
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; 0x200],
    m_waveform: [[u16; 0x400]; 8],
}

// Function to extract a bitfield from a 32-bit value
fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let mask = ((1 << length as u32) - 1) as u32;
    (value >> start as u32) & mask
}

// Function to extract a byte from the registers
fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    opl_emu_bitfield(regs.m_regdata[(offset + extra_offset) as usize] as u32, start as i32, count as i32)
}

// Function to check the new flag in the registers
fn opl_emu_registers_newflag(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x105, 0, 1, 0)
}

// Function to get the channel output from the registers
fn opl_emu_registers_ch_output_2(regs: &OplEmuRegisters, choffs: u32) -> u32 {
    if opl_emu_registers_newflag(regs) != 0 {
        opl_emu_registers_byte(regs, 0xc0 + choffs, 6, 1, 0)
    } else {
        0
    }
}
