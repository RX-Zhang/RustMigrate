
use std::convert::TryInto;
use std::ffi::CString;
use std::os::raw::c_char;

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

extern "C" {
    fn opl_load_op2_voice(timbre: &mut OplTimbre, buff: *const u8, len: usize);
}

fn opl_load_op2_voice_rust(timbre: &mut OplTimbre, buff: &[u8]) {
    if buff.len() != 16 {
        return;
    }
    unsafe {
        opl_load_op2_voice(timbre, buff.as_ptr(), buff.len());
    }
}

fn main() {
    let timbre = &mut OplTimbre {
        modulator_E862: 0,
        modulator_40: 0,
        carrier_40: 0,
        feedconn: 0,
        finetune: 0,
        notenum: 0,
        noteoffset: 0,
        carrier_E862: 0,
    };

    let buff = &[
        0x00, 0x00, 0x00, 0x07, 0x3e, 0x00, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];

    opl_load_op2_voice_rust(timbre, buff);
}
