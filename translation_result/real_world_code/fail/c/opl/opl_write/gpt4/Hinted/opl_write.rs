
struct OplEmuFmOperator {
    m_keyon_live: u32,
}

enum OplEmuKeyonType {
    Type1,
    Type2,
    // Add other types as needed
}

fn opl_emu_bitfield(value: u32, start: u8, len: u8) -> u32 {
    (value >> start) & ((1 << len) - 1)
}

fn opl_emu_fm_operator_keyonoff(fmop: &mut OplEmuFmOperator, on: u32, type_: OplEmuKeyonType) {
    let type_index = type_ as u8;
    fmop.m_keyon_live = (fmop.m_keyon_live & !(1 << type_index)) 
        | (opl_emu_bitfield(on, 0, 1) << type_index);
}
