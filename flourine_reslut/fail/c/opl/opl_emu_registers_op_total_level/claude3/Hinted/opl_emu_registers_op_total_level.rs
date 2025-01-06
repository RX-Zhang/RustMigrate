
use std::collections::HashMap;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS]>,
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>,
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(
    regs: &OplEmuRegisters,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    opl_emu_bitfield(
        regs.m_regdata[(offset + extra_offset) as usize] as u32,
        start as i32,
        count as i32,
    )
}

fn opl_emu_registers_op_total_level(regs: &OplEmuRegisters, opoffs: u32) -> u32 {
    opl_emu_registers_byte(regs, 0x40, 0, 6, opoffs)
}

fn main() {
    let mut regs = OplEmuRegisters {
        m_lfo_am_counter: 16448,
        m_lfo_pm_counter: 6168,
        m_noise_lfsr: 0,
        m_lfo_am: 0,
        m_regdata: Box::new([0; OPL_EMU_REGISTERS_REGISTERS]),
        m_waveform: Box::new([[0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]),
    };

    regs.m_waveform[0][3] = 65280;
    regs.m_waveform[0][4] = 65344;
    regs.m_waveform[1][3] = 255;
    regs.m_waveform[2][0] = 65280;
    regs.m_waveform[2][1] = 65535;
    regs.m_waveform[2][2] = 65535;
    regs.m_waveform[2][3] = 65535;
    regs.m_waveform[2][4] = 65535;
    regs.m_waveform[4][4] = 16639;

    let opoffs = 196608;
    let result = opl_emu_registers_op_total_level(&regs, opoffs);
    println!("{}", result);
}
