
use std::convert::TryInto;
use std::mem;
use std::ops::Shr;

#[repr(C)]
struct OplTimbre {
    modulator_e862: u32,
    modulator_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
    carrier_e862: u32,
    carrier_40: u8,
}

impl OplTimbre {
    fn new() -> Self {
        OplTimbre {
            modulator_e862: 0,
            modulator_40: 0,
            feedconn: 0,
            finetune: 0,
            notenum: 0,
            noteoffset: 0,
            carrier_e862: 0,
            carrier_40: 0,
        }
    }
}

fn opl_load_op2_voice(timbre: &mut OplTimbre, buff: &[u8]) {
    timbre.modulator_e862 = u32::from_le_bytes([buff[2], buff[1], buff[0], 0])
        .wrapping_shr((24 - 7) as u32)
        .wrapping_shl(7);
    timbre.carrier_e862 = u32::from_le_bytes([buff[7], buff[6], buff[5], 0])
        .wrapping_shr((24 - 7) as u32)
        .wrapping_shl(7);
    timbre.modulator_40 = (buff[4].wrapping_shr(6) & 0x3F) as u8 | (buff[5] & 0xC0);
    timbre.carrier_40 = (buff[11].wrapping_shr(6) & 0x3F) as u8 | (buff[12] & 0xC0);
    timbre.feedconn = buff[6];
    timbre.finetune = 0;
    let noteoffset_bytes = [buff[15], buff[14]];
    timbre.noteoffset = i16::from_le_bytes(noteoffset_bytes);
}
