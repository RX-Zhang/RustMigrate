
use std::ops::{Shr, Shl, BitOr, BitAnd};

#[derive(Copy, Clone)]
struct OplEmuFmOperator {
    m_choffs: u32,
    m_mul: u32,
    m_tl: u32,
    m_wave: u32,
    m_feedback: u32,
    m_conn: u32,
    m_ksl: u32,
    m_ams: u32,
    m_cont: u32,
}

fn opl_midi_changeprog(op: &mut OplEmuFmOperator, newprog: u8) {
    let ksl = match newprog & 0x0e {
        0x00 => 0x0,
        0x02 => 0x1,
        0x04 => 0x2,
        0x06 => 0x3,
        0x08 => 0x4,
        0x0a => 0x5,
        0x0c => 0x6,
        0x0e => 0x7,
        _ => unreachable!(),
    };

    let am = match (newprog & 0xf0) >> 4 {
        0x0 => 0x0,
        0x1 => 0x1,
        0x2 => 0x2,
        0x3 => 0x3,
        0x4 => 0x4,
        0x5 => 0x5,
        0x6 => 0x6,
        0x7 => 0x7,
        _ => unreachable!(),
    };

    op.m_ksl = (op.m_ksl & !(0x7 << 24)) | ((ksl as u32) << 24);
    op.m_ams = (op.m_ams & !(0x7 << 16)) | ((am as u32) << 16);
}
