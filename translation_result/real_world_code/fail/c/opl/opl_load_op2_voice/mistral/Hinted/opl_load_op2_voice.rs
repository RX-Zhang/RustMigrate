
use std::u8;

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

impl OplTimbre {
    fn load_op2_voice(&mut self, buff: &[u8]) {
        self.modulator_E862 = u32::from_le_bytes([buff[2], buff[1], buff[0], 0])
            .wrapping_shl(8)
            .wrapping_shl(8)
            .wrapping_shl(8)
            .wrapping_shl(8);
        self.carrier_E862 = u32::from_le_bytes([buff[7], buff[8], buff[9], buff[10]])
            .wrapping_shl(8)
            .wrapping_shl(8)
            .wrapping_shl(8)
            .wrapping_shl(8);
        self.modulator_40 = (buff[4] & 0xc0) as u8 | (buff[5] & 0x3f);
        self.carrier_40 = (buff[11] & 0xc0) as u8 | (buff[12] & 0x3f);
        self.feedconn = buff[6];
        self.finetune = 0;
        self.noteoffset = i16::from_le_bytes([buff[15], buff[14]]);
    }
}
