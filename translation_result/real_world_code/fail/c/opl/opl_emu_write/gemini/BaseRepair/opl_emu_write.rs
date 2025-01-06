
// Define the enum for OplEmuKeyonType
enum OplEmuKeyonType {
    // Add relevant variants here, e.g.
    KeyOn,
    KeyOff,
    // Other variants if needed
}

// Define the struct for OplEmuFmOperator
struct OplEmuFmOperator {
    m_keyon_live: u8,
}

// Function implementation
fn opl_emu_fm_operator_keyonoff(fmop: &mut Box<OplEmuFmOperator>, on: u32, type_: OplEmuKeyonType) {
    let type_as_i32 = match type_ {
        OplEmuKeyonType::KeyOn => 0,
        OplEmuKeyonType::KeyOff => 1,
        // Add other variants as needed
    };
    fmop.m_keyon_live = (fmop.m_keyon_live & !(1 << type_as_i32)) | (((on % 2) << type_as_i32) as u8);
}
