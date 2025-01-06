
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
pub fn opl_emu_bitfield(value: u32, start: usize, length: usize) -> u32 {
    (value >> (start % 32)) & ((1 << (length % 32)) - 1)
}

// Function to extract a byte from the registers
pub fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: usize, start: usize, count: usize, extra_offset: usize) -> u32 {
    let reg_index = (offset.wrapping_add(extra_offset)) % OPL_EMU_REGISTERS_REGISTERS;
    let reg_value = regs.m_regdata[reg_index] as u32;
    opl_emu_bitfield(reg_value, start, count)
}
