

use std::num::Wrapping;

struct OplTimbre {
    modulator_e862: Wrapping<u32>,
    carrier_e862: Wrapping<u32>,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16
}

fn opl_load_op2_voice(timbre: &mut OplTimbre, buff: &[u8; 16]) {
    if buff.len() != 16 {
        panic!("Invalid input length");
    }

    /* load modulator */
    timbre.modulator_e862 = Wrapping(buff[3] as u32); /* wave select */
    timbre.modulator_e862 += Wrapping((buff[2] as u32) << 8); /* sust/release */
    timbre.modulator_e862 += Wrapping((buff[1] as u32) << 16); /* attack/decay */
    timbre.modulator_e862 += Wrapping((buff[0] as u32) << 24); /* AM/VIB... flags */

    /* load carrier */
    timbre.carrier_e862 = Wrapping(buff[10] as u32); /* wave select */
    timbre.carrier_e862 += Wrapping((buff[9] as u32) << 8); /* sust/release */
    timbre.carrier_e862 += Wrapping((buff[8] as u32) << 16); /* attack/decay */
    timbre.carrier_e862 += Wrapping((buff[7] as u32) << 24); /* AM/VIB... flags */

    /* load KSL */
    timbre.modulator_40 = buff[5] & 0x3f | buff[4] & 0xc0;
    timbre.carrier_40 = buff[12] & 0x3f | buff[11] & 0xc0;

    /* feedconn & finetune */
    timbre.feedconn = buff[6];
    timbre.finetune = 0;
    
    timbre.noteoffset = i16::from_le_bytes([buff[14], buff[15]]);
}

