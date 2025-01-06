
// Constants
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

// Struct definition
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

// Function to extract a bitfield
fn opl_emu_bitfield(value: u32, start: usize, length: usize) -> u32 {
    let mask = (1 << length) - 1;
    (value >> start) & mask as u32
}

// Function to get a byte from the registers
fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: usize, start: usize, count: usize, extra_offset: usize) -> u32 {
    if offset + extra_offset >= OPL_EMU_REGISTERS_REGISTERS {
        return 0;
    }
    opl_emu_bitfield(regs.m_regdata[offset + extra_offset] as u32, start, count)
}

// Function to get the new flag from the registers
fn opl_emu_registers_newflag(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x105, 0, 1, 0)
}

// Function to get the channel output from the registers
fn opl_emu_registers_ch_output_2(regs: &OplEmuRegisters, choffs: u32) -> u32 {
    if opl_emu_registers_newflag(regs) != 0 {
        opl_emu_registers_byte(regs, 0xc0, 6, 1, choffs as usize)
    } else {
        0
    }
}
