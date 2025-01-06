

use std::mem;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[repr(u8)]
enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
}

const OPL_EMU_EG_STATES: u8 = 6;

struct OplEmuFmOperator;

impl OplEmuFmOperator {
    fn opl_emu_bitfield(&self, value: u32, start: i32, length: i32) -> u32 {
        (value >> start) & ((1 << length) - 1)
    }

    fn opl_emu_attenuation_increment(&self, rate: u32, index: u32) -> u32 {
        let s_increment_table = [
            0x00000000, 0x00000000, 0x10101010, 0x10101010,  // 0-3    (0x00-0x03)
            // ...
            0x88888888, 0x88888888, 0x88888888, 0x88888888,   // 56-59  (0x38-0x3B)
            0x88888888,
        ];
        self.opl_emu_bitfield(s_increment_table[rate as usize], 4 * index as i32, 4)
    }

    fn opl_emu_fm_operator_clock_envelope(&mut self, env_counter: u32) {
        // ...
    }
}

