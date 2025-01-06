

// Define constants
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

// Define the struct
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

// Function to extract a bitfield
fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let mask = (1 << length as u32) - 1;
    (value >> start as u32) & mask
}

// Function to get a byte from the registers
fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    opl_emu_bitfield(regs.m_regdata[offset as usize + extra_offset as usize] as u32, start as i32, count as i32)
}

// Function to select a note from the registers
fn opl_emu_registers_note_select(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x08, 6, 1, 0)
}
