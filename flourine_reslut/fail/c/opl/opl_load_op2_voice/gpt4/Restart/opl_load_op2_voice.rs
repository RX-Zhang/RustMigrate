
use std::mem::MaybeUninit;

#[derive(Debug, Default)]
struct OplTimbreT {
    modulator_E862: u32,
    carrier_E862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
}

fn opl_load_op2_voice(timbre: &mut OplTimbreT, buff: &[u8]) {
    // Load modulator
    timbre.modulator_E862 = buff[3] as u32; // wave select
    timbre.modulator_E862 = timbre.modulator_E862.wrapping_shl(8);
    timbre.modulator_E862 |= buff[2] as u32; // sust/release
    timbre.modulator_E862 = timbre.modulator_E862.wrapping_shl(8);
    timbre.modulator_E862 |= buff[1] as u32; // attack/decay
    timbre.modulator_E862 = timbre.modulator_E862.wrapping_shl(8);
    timbre.modulator_E862 |= buff[0] as u32; // AM/VIB... flags
    // Load carrier
    timbre.carrier_E862 = buff[10] as u32; // wave select
    timbre.carrier_E862 = timbre.carrier_E862.wrapping_shl(8);
    timbre.carrier_E862 |= buff[9] as u32; // sust/release
    timbre.carrier_E862 = timbre.carrier_E862.wrapping_shl(8);
    timbre.carrier_E862 |= buff[8] as u32; // attack/decay
    timbre.carrier_E862 = timbre.carrier_E862.wrapping_shl(8);
    timbre.carrier_E862 |= buff[7] as u32; // AM/VIB... flags
    // Load KSL
    timbre.modulator_40 = (buff[5] & 0x3f) | (buff[4] & 0xc0);
    timbre.carrier_40 = (buff[12] & 0x3f) | (buff[11] & 0xc0);
    // feedconn & finetune
    timbre.feedconn = buff[6];
    timbre.finetune = 0;
    timbre.noteoffset = i16::from_le_bytes([buff[14], buff[15]]);
}

fn main() {
    let mut timbre = OplTimbreT::default();
    let buff: Vec<u8> = vec![0; 16]; // Example buffer, replace with actual data
    opl_load_op2_voice(&mut timbre, &buff);
    println!("{:?}", timbre);
}
