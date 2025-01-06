
enum OplEmuKeyonType {
    Type1,
    Type2,
    // Add other variants as needed
}

struct OplEmuFmOperator {
    m_keyon_live: u8,
}

fn opl_emu_fm_operator_keyonoff(fmop: &mut OplEmuFmOperator, on: u32, r#type: OplEmuKeyonType) {
    let bitfield_value: u32 = on & 0x01; // Standalone function replacement for 'opl_emu_bitfield(on, 0, 1)'
    let type_index = r#type as usize;
    fmop.m_keyon_live = ((fmop.m_keyon_live as u32) & !(1 << type_index) | (bitfield_value << type_index)) as u8;
}
