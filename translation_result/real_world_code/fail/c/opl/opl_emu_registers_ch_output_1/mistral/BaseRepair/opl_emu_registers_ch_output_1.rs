

use std::u16;
use std::u32;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[repr(C)]
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

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    let index = offset as usize + extra_offset as usize;
    if index < OPL_EMU_REGISTERS_REGISTERS {
        let value = regs.m_regdata[index] as u32;
        opl_emu_bitfield(value, start, count)
    } else {
        0
    }
}

fn opl_emu_registers_newflag(regs: &OplEmuRegisters) -> bool {
    let value = opl_emu_registers_byte(regs, 0x105, 0, 1, 0);
    value != 0
}

fn opl_emu_registers_ch_output_1(regs: &OplEmuRegisters, choffs: u32) -> u32 {
    if opl_emu_registers_newflag(regs) {
        let index = (0xc0 + choffs) as usize;
        if index < OPL_EMU_REGISTERS_REGISTERS {
            let value = regs.m_regdata[index] as u32;
            opl_emu_bitfield(value, 5, 1)
        } else {
            0
        }
    } else {
        1
    }
}

