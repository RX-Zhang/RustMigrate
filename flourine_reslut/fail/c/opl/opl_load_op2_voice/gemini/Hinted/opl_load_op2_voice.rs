
use std::convert::TryInto;

#[repr(C)]
#[derive(Copy, Clone)]
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
    // load modulator
    timbre.modulator_e862 = (buff[3] as u32) << 16;
    timbre.modulator_e862 |= (buff[2] as u32) << 8;
    timbre.modulator_e862 |= buff[1] as u32;
    timbre.modulator_e862 |= (buff[0] as u32) << 24;
    // load carrier
    timbre.carrier_e862 = (buff[10] as u32) << 16;
    timbre.carrier_e862 |= (buff[9] as u32) << 8;
    timbre.carrier_e862 |= buff[8] as u32;
    timbre.carrier_e862 |= (buff[7] as u32) << 24;
    // load KSL
    timbre.modulator_40 = (buff[5] & 0x3f) | ((buff[4] & 0xc0) >> 2);
    timbre.carrier_40 = (buff[12] & 0x3f) | ((buff[11] & 0xc0) >> 2);
    // feedconn & finetune
    timbre.feedconn = buff[6];
    timbre.finetune = 0;
    timbre.noteoffset = i16::from_be_bytes(buff[14..16].try_into().unwrap());
}
