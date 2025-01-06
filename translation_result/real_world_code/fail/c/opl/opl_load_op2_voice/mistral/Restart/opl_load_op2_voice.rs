
use std::convert::TryInto;

#[repr(C)]
struct OplTimbre {
    modulator_E862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
    carrier_E862: u32,
}

fn load_op2_voice(timbre: &mut OplTimbre, buff: &[u8]) {
    timbre.modulator_E862 = (u32::from_le_bytes([buff[2], buff[1], buff[0], 0]) & 0xFFFFFF00)
        .wrapping_shl(8)
        .wrapping_shl(8)
        .wrapping_shl(8)
        .wrapping_shl(8);
    timbre.carrier_E862 = (u32::from_le_bytes([buff[7], buff[6], buff[5], 0]) & 0x00FF0000)
        .wrapping_shl(24)
        .wrapping_shl(8)
        .wrapping_shl(8)
        .wrapping_shl(8);
    timbre.modulator_40 = (buff[4] & 0xC0) | (buff[5] & 0x3F);
    timbre.carrier_40 = (buff[11] & 0xC0) | (buff[12] & 0x3F);
    timbre.feedconn = buff[6];
    timbre.finetune = 0;
    timbre.noteoffset = i16::from_le_bytes([buff[15], buff[14]]);
}
