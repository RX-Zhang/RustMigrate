
use std::convert::TryInto;
use std::i16;
use std::i32;
use std::u8;

#[repr(C)]
struct OplTimbre {
    modulator_E862: i32,
    carrier_E862: i32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
}

fn opl_load_op2_voice(timbre: &mut OplTimbre, buff: &[u8]) {
    timbre.modulator_E862 = i32::from(buff[3])
        | (i32::from(buff[2]) << 8)
        | (i32::from(buff[1]) << 16)
        | (i32::from(buff[0]) << 24);

    timbre.carrier_E862 = i32::from(buff[7])
        | (i32::from(buff[6]) << 8)
        | (i32::from(buff[5]) << 16)
        | (i32::from(buff[4]) << 24);

    timbre.modulator_40 = (buff[4] & 0x3f) | ((buff[5] as u16 & 0xc0) as u8);
    timbre.carrier_40 = (buff[11] & 0x3f) | ((buff[12] as u16 & 0xc0) as u8);

    timbre.feedconn = buff[6];
    timbre.finetune = 0;

    let offset_bytes: [u8; 2] = [buff[15], buff[14]];
    timbre.noteoffset = i16::from_le_bytes(offset_bytes);
}
