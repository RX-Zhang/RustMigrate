
const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct Opl {
    channelprog: [u8; 16],
    is_op2: bool,
}

fn get_instrument(opl: &mut Opl, channel: usize, note: i8) -> i32 {
    if (note < 0) || (note > 127) || (channel > 15) {
        return -1;
    }

    if channel == 9 {
        return if opl.is_op2 {
            128 + note as i32 - 35
        } else {
            128 | note as i32
        };
    }

    return opl.channelprog[channel] as i32;
}
