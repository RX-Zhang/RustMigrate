
use std::u16;
use std::u32;
use std::mem;

const OPL_EMU_REGISTERS_RHYTHM_CHANNEL: u16 = 0xff;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_REG_MODE: u16 = 0x04;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u32,
    m_lfo_pm_counter: u32,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_write(
    regs: &mut OplEmuRegisters,
    index: u16,
    data: u8,
    channel: &mut u32,
    opmask: &mut u32,
) -> bool {
    if index == OPL_EMU_REGISTERS_REG_MODE && opl_emu_bitfield(data as u32, 7, 1) != 0 {
        regs.m_regdata[index as usize] |= 0x80;
    } else {
        regs.m_regdata[index as usize] = data;
    }

    *channel = if index == 0xbd {
        if opl_emu_bitfield(data as u32, 5, 1) != 0 {
            opl_emu_bitfield(data as u32, 0, 5) as u32
        } else {
            0
        }
    } else if (index & 0xf0) == 0xb0 && (index as usize) < 0xb9 {
        let ch = (index & 0x0f) as u32;
        if ch < 9 {
            ch + (9 * opl_emu_bitfield(index as u32, 8, 1) as u32)
        } else {
            0
        }
    } else {
        0
    };

    *opmask = if (index & 0xf0) == 0xb0 && (index as usize) < 0xb9 {
        if opl_emu_bitfield(data as u32, 5, 1) != 0 {
            0xf
        } else {
            0
        }
    } else {
        0
    };

    *channel != 0 || *opmask != 0
}

