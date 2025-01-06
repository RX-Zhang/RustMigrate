

use std::convert::TryInto;
use std::mem;
use std::ops::{Shl, Shr};

const OPL_EMU_REGISTERS_CHANNELS: u16 = 18;
const OPL_EMU_REGISTERS_REGISTERS: u16 = 0x200;
const OPL_EMU_REGISTERS_REG_MODE: u16 = 0x04;
const OP2_2NDVOICE_PRIORITY_PENALTY: u8 = 0xFF;

// ... (other enums and consts definitions)

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u8,
}

// ... (other function definitions)

fn set_opl_emu_register(
    registers: &mut Box<[u8]>,
    address: u16,
    value: u8,
) {
    if address > OPL_EMU_REGISTERS_REGISTERS {
        return;
    }

    let index = address as usize;
    registers[index] = value;
}

fn get_opl_emu_register(
    registers: &Box<[u8]>,
    address: u16,
) -> u8 {
    if address > OPL_EMU_REGISTERS_REGISTERS {
        return 0;
    }

    let index = address as usize;
    registers[index]
}

fn opl_emu_register_write(
    registers: &mut Box<[u8]>,
    address: u16,
    value: u8,
) {
    if address > OPL_EMU_REGISTERS_REGISTERS {
        return;
    }

    let index = address as usize;
    registers[index] = value;

    // Handle special registers
    match address {
        OPL_EMU_REGISTERS_REG_MODE => {
            // ... (special register handling)
        }
        _ => {}
    }
}

fn opl_emu_register_read(
    registers: &Box<[u8]>,
    address: u16,
) -> u8 {
    if address > OPL_EMU_REGISTERS_REGISTERS {
        return 0;
    }

    let index = address as usize;
    registers[index]
}

fn opl_emu_register_peek(
    registers: &Box<[u8]>,
    address: u16,
) -> u8 {
    if address > OPL_EMU_REGISTERS_REGISTERS {
        return 0;
    }

    let index = address as usize;
    registers[index]
}

fn calculate_opl_emu_registers_all_channels() -> u32 {
    (1 as u32).wrapping_shl(OPL_EMU_REGISTERS_CHANNELS.try_into().unwrap()) - 1
}

