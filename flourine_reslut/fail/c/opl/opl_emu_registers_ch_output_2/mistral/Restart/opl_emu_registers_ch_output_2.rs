
use libc::{c_int, c_uint};
use std::mem;

const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x100;

pub struct OplEmuRegisters {
    waveforms: [u16; OPL_EMU_REGISTERS_WAVEFORMS as usize],
    registers: Box<[u8; OPL_EMU_REGISTERS_REGISTERS as usize]>,
    waveform_length: Box<[u8; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]>,
}

impl OplEmuRegisters {
    pub fn new() -> Self {
        OplEmuRegisters {
            waveforms: [0; OPL_EMU_REGISTERS_WAVEFORMS as usize],
            registers: Box::new([0; OPL_EMU_REGISTERS_REGISTERS as usize]),
            waveform_length: Box::new([0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]),
        }
    }

    pub fn get_waveform_mut(&mut self, index: u32) -> &mut [u16; 8] {
        if index >= OPL_EMU_REGISTERS_WAVEFORMS {
            panic!("Invalid waveform index");
        }
        &mut self.waveforms
    }

    pub fn get_registers_mut(&mut self) -> &mut [u8; OPL_EMU_REGISTERS_REGISTERS as usize] {
        &mut self.registers
    }

    pub fn get_waveform_length_mut(&mut self, index: u32) -> &mut [u8; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize] {
        if index >= OPL_EMU_REGISTERS_WAVEFORMS {
            panic!("Invalid waveform length index");
        }
        &mut self.waveform_length
    }
}
