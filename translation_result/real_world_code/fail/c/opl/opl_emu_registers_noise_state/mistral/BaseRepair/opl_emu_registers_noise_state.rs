
use std::ops::Shr;
use std::u32;

const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 0x100;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x40;

fn read_register(address: u32) -> u32 {
    if address < OPL_EMU_REGISTERS_REGISTERS {
        address
    } else {
        0
    }
}

fn write_register(address: u32, value: u32) {
    if address < OPL_EMU_REGISTERS_REGISTERS {
        // Do something with the value here
    }
}

fn read_waveform_length(address: u32) -> u32 {
    if (address >= OPL_EMU_REGISTERS_WAVEFORMS) & (address < (OPL_EMU_REGISTERS_WAVEFORMS + OPL_EMU_REGISTERS_WAVEFORM_LENGTH)) {
        address
    } else {
        0
    }
}

fn write_waveform_length(address: u32, value: u32) {
    if (address >= OPL_EMU_REGISTERS_WAVEFORMS) & (address < (OPL_EMU_REGISTERS_WAVEFORMS + OPL_EMU_REGISTERS_WAVEFORM_LENGTH)) {
        // Do something with the value here
    }
}
