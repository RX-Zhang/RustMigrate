
use std::convert::TryInto;
use std::mem;

const OPL_EMU_REGISTERS_RHYTHM_CHANNEL: u16 = 0xff;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_REG_MODE: u16 = 0x04;
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

fn opl_emu_bitfield(value: u32, start: i32, length: u32) -> u32 {
    (value >> (start as u32)) & ((1 << length) - 1)
}

fn opl_emu_registers_write(
    regs: &mut OplEmuRegisters,
    index: u16,
    data: u8,
    channel: &mut u32,
    opmask: &mut u32,
) {
    if index == OPL_EMU_REGISTERS_REG_MODE {
        *opmask = (data as u32) << 24;
    } else {
        let reg_index = (index as usize) + (9 * (*channel as usize));
        if reg_index < OPL_EMU_REGISTERS_REGISTERS {
            regs.m_regdata[reg_index] = data;
        }
    }
}
