
struct OplEmuFmOperator {
    m_keyon_live: u8,
}

enum OplEmuKeyonType {
    Type1 = 0,
    Type2 = 1,
    // Add other types as needed
}

fn opl_emu_bitfield(value: u32, shift: u32, width: u32) -> u32 {
    (value >> shift) & ((1 << width) - 1)
}

fn opl_emu_fm_operator_keyonoff(fmop: &mut OplEmuFmOperator, on: u32, type_: OplEmuKeyonType) {
    let type_val = type_ as u8;
    fmop.m_keyon_live = (fmop.m_keyon_live & !(1 << type_val))
        | ((opl_emu_bitfield(on, 0, 1) as u8) << type_val);
}
