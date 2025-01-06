
use std::mem;

#[repr(C)]
struct OplTimbre {
    modulator_e862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
    carrier_e862: u32,
}

impl OplTimbre {
    fn new() -> Self {
        OplTimbre {
            modulator_e862: 0,
            modulator_40: 0,
            carrier_40: 0,
            feedconn: 0,
            finetune: 0,
            notenum: 0,
            noteoffset: 0,
            carrier_e862: 0,
        }
    }
}
