
use std::convert::TryInto;

#[derive(Debug)]
pub struct OplTimbre {
    pub modulator_e862: u32,
    pub carrier_e862: u32,
    pub modulator_40: u8,
    pub carrier_40: u8,
    pub feedconn: u8,
    pub finetune: i8,
    pub notenum: u8,
    pub noteoffset: i16,
}

pub fn opl_load_op2_voice(timbre: &mut OplTimbre, buff: &[u8]) {
    timbre.modulator_e862 = u32::from_le_bytes(buff[0..4].try_into().unwrap());
    timbre.carrier_e862 = u32::from_le_bytes(buff[7..11].try_into().unwrap());
    timbre.modulator_40 = (buff[4] & 0xc0) | (buff[5] & 0x3f);
    timbre.carrier_40 = (buff[11] & 0xc0) | (buff[12] & 0x3f);
    timbre.feedconn = buff[6];
    timbre.finetune = 0;
    timbre.noteoffset = i16::from_le_bytes(buff[14..16].try_into().unwrap());
}
