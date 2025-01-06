
use std::convert::TryInto;

#[repr(C)]
struct OplTimbre {
    modulator_e862: u32,
    carrier_e862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
}

fn opl_load_op2_voice(timbre: &mut OplTimbre, buff: &[u8; 16]) {
    /* load modulator */
    timbre.modulator_e862 = u32::from(buff[3]); /* wave select */
    timbre.modulator_e862 <<= 8;
    timbre.modulator_e862 |= u32::from(buff[2]); /* sust/release */ 
    timbre.modulator_e862 <<= 8;
    timbre.modulator_e862 |= u32::from(buff[1]); /* attack/decay */
    timbre.modulator_e862 <<= 8;
    timbre.modulator_e862 |= u32::from(buff[0]); /* AM/VIB... flags */
    
    /* load carrier */
    timbre.carrier_e862 = u32::from(buff[10]); /* wave select */
    timbre.carrier_e862 <<= 8; 
    timbre.carrier_e862 |= u32::from(buff[9]); /* sust/release */
    timbre.carrier_e862 <<= 8;
    timbre.carrier_e862 |= u32::from(buff[8]); /* attack/decay */
    timbre.carrier_e862 <<= 8;
    timbre.carrier_e862 |= u32::from(buff[7]); /* AM/VIB... flags */
    
    /* load KSL */
    timbre.modulator_40 = buff[5] & 0x3F | buff[4] & 0xC0; 
    timbre.carrier_40 = buff[12] & 0x3F | buff[11] & 0xC0;
    
    /* feedconn & finetune */
    timbre.feedconn = buff[6];
    timbre.finetune = 0;
    
    timbre.noteoffset = i16::from(buff[14]) | (i16::from(buff[15]) << 8);
}
