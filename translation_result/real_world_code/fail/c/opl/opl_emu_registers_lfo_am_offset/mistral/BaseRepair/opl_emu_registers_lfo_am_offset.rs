
use std::u16;
use std::u32;
use std::u8;
use std::ops::{Shr, Add};

const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x400;

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_am_waveform: u16,
    m_lfo_am_enabled: bool,
    m_lfo_fm_counter: u16,
    m_lfo_fm_waveform: u16,
    m_lfo_fm_enabled: bool,
    m_feedback_enabled: bool,
    m_feedback_value: u8,
    m_amp_mod_enabled: bool,
    m_amp_mod_value: u8,
    m_waveform_length: u16,
    m_waveform_data: [u8; 32],
}

impl OplEmuRegisters {
    fn new() -> Self {
        OplEmuRegisters {
            m_lfo_am_counter: 0,
            m_lfo_am_waveform: 0,
            m_lfo_am_enabled: false,
            m_lfo_fm_counter: 0,
            m_lfo_fm_waveform: 0,
            m_lfo_fm_enabled: false,
            m_feedback_enabled: false,
            m_feedback_value: 0,
            m_amp_mod_enabled: false,
            m_amp_mod_value: 0,
            m_waveform_length: 0,
            m_waveform_data: [0; 32],
        }
    }

    fn lfo_am_counter(&self) -> u16 {
        self.m_lfo_am_counter
    }

    fn set_lfo_am_counter(&mut self, value: u16) {
        self.m_lfo_am_counter = value;
    }

    fn lfo_am_enabled(&self) -> bool {
        self.m_lfo_am_enabled
    }

    fn set_lfo_am_enabled(&mut self, value: bool) {
        self.m_lfo_am_enabled = value;
    }

    fn lfo_fm_counter(&self) -> u16 {
        self.m_lfo_fm_counter
    }

    fn set_lfo_fm_counter(&mut self, value: u16) {
        self.m_lfo_fm_counter = value;
    }

    fn lfo_fm_enabled(&self) -> bool {
        self.m_lfo_fm_enabled
    }

    fn set_lfo_fm_enabled(&mut self, value: bool) {
        self.m_lfo_fm_enabled = value;
    }

    fn feedback_enabled(&self) -> bool {
        self.m_feedback_enabled
    }

    fn set_feedback_enabled(&mut self, value: bool) {
        self.m_feedback_enabled = value;
    }

    fn feedback_value(&self) -> u8 {
        self.m_feedback_value
    }

    fn set_feedback_value(&mut self, value: u8) {
        self.m_feedback_value = value;
    }

    fn amp_mod_enabled(&self) -> bool {
        self.m_amp_mod_enabled
    }

    fn set_amp_mod_enabled(&mut self, value: bool) {
        self.m_amp_mod_enabled = value;
    }

    fn amp_mod_value(&self) -> u8 {
        self.m_amp_mod_value
    }

    fn set_amp_mod_value(&mut self, value: u8) {
        self.m_amp_mod_value = value;
    }

    fn waveform_length(&self) -> u16 {
        self.m_waveform_length
    }

    fn set_waveform_length(&mut self, value: u16) {
        self.m_waveform_length = value;
    }

    fn waveform_data(&self) -> &[u8; 32] {
        &self.m_waveform_data
    }

    fn waveform_data_mut(&mut self) -> &mut [u8; 32] {
        &mut self.m_waveform_data
    }
}

fn wrap_add(value: u16, max: u16) -> u16 {
    (value + 1).wrapping_add(max).wrapping_sub(1)
}

fn shift_right(value: u16, shift: u8) -> u16 {
    (value as u32).shr(shift as u32) as u16
}
