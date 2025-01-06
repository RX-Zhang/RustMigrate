

use std::boxed::Box;
use std::convert::TryInto;

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8; 512]>,
    m_waveform: Box<[[u16; 1024]; 8]>,
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_write(
    regs: &mut OplEmuRegisters,
    index: u16,
    data: u8,
    channel: &mut u32,
    opmask: &mut u32
) -> i32 {
    if index == 4 && opl_emu_bitfield(data as u32, 7, 1) != 0 {
        regs.m_regdata[index as usize] |= 0x80;
    } else {
        regs.m_regdata[index as usize] = data;
    }

    if index == 189 {
        *channel = 255;
        *opmask = if opl_emu_bitfield(data as u32, 5, 1) != 0 {
            opl_emu_bitfield(data as u32, 0, 5)
        } else {
            0
        };
        return 1;
    }

    if (index & 0xF0) == 0xB0 {
        *channel = (index.wrapping_add(9) as u32) & 0x0F;
        if *channel < 9 {
            *channel = (*channel as u16).wrapping_add(9) as u32;
            *opmask = if opl_emu_bitfield(data as u32, 5, 1) != 0 {
                15
            } else {
                0
            };
            return 1;
        }
    }

    0
}

