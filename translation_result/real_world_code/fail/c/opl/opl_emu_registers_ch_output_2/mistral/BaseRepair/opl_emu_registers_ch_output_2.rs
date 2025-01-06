

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

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: usize, start: u32, count: u32, extra_offset: usize) -> u32 {
    let real_offset = offset + extra_offset;
    if real_offset >= OPL_EMU_REGISTERS_REGISTERS {
        return 0;
    }
    let value = regs.m_regdata[real_offset] as u32;
    opl_emu_bitfield(value, start, count)
}

fn opl_emu_registers_newflag(regs: &OplEmuRegisters) -> bool {
    opl_emu_registers_byte(regs, 0x105 as usize, 0, 1, 0) != 0
}

fn opl_emu_registers_ch_output_2(regs: &OplEmuRegisters, choffs: usize) -> u32 {
    if opl_emu_registers_newflag(regs) {
        let real_offset = 0xc0 + choffs;
        if real_offset >= OPL_EMU_REGISTERS_REGISTERS {
            return 0;
        }
        opl_emu_registers_byte(regs, real_offset, 6, 1, 0)
    } else {
        0
    }
}

