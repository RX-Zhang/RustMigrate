

use std::mem;
use std::ops::BitAnd;
use std::ptr;

const OPL_EMU_REGISTERS_RHYTHM_CHANNEL: u16 = 0xff;
const OPL_EMU_REGISTERS_WAVEFORMS: u8 = 8;
const OPL_EMU_REGISTERS_REGISTERS: u16 = 0x200;
const OPL_EMU_REGISTERS_REG_MODE: u16 = 0x04;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u16 = 0x400;

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS as usize],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]; OPL_EMU_REGISTERS_WAVEFORMS as usize],
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_write(
    regs: &mut OplEmuRegisters,
    index: u16,
    data: u8,
    channel: &mut u32,
    opmask: &mut u32,
) -> i32 {
    if index == OPL_EMU_REGISTERS_REG_MODE && opl_emu_bitfield(data as u32, 7, 1) != 0 {
        regs.m_regdata[index as usize] |= 0x80;
    } else {
        regs.m_regdata[index as usize] = data;
    }

    let mut result = 0;

    if index == 0xbd {
        *channel = u32::from(OPL_EMU_REGISTERS_RHYTHM_CHANNEL);
        *opmask = ((data as u32) & 0x1f).bitand(0x1f);
        result = 1;
    } else if (index & 0xf0) == 0xb0 {
        *channel = (index & 0x0f) as u32;
        if *channel < 9 {
            *channel += 9 * opl_emu_bitfield(index as u32, 8, 1) as u32;
            *opmask = ((data as u32) & 0x1f).bitand(0x1f);
            result = 1;
        }
    }

    result
}

