
enum OplEmuKeyonType {
    Type0,
    Type1,
    Type2,
    Type3,
}

struct OplEmuFmOperator {
    m_keyon_live: u32,
}

fn opl_emu_bitfield(value: u32, offset: u32, size: u32) -> u32 {
    (value >> offset) & ((1 << size) - 1)
}

fn opl_emu_fm_operator_keyonoff(fmop: &mut OplEmuFmOperator, on: u32, type_: OplEmuKeyonType) {
    let type_val = type_ as u32;
    fmop.m_keyon_live = (fmop.m_keyon_live & !(1 << type_val)) | (opl_emu_bitfield(on, 0, 1) << type_val);
}
