
enum OpTwoFlags {
    FixedPitch,
    Unused,
    DoubleVoice,
}

fn op_two_flags_from_bits(bits: u16) -> OpTwoFlags {
    match bits {
        1 => OpTwoFlags::FixedPitch,
        2 => OpTwoFlags::Unused,
        4 => OpTwoFlags::DoubleVoice,
        _ => panic!("Invalid OpTwoFlags bits"),
    }
}

fn opl_loadbank_op2(opl: &mut [OpTwoFlags; 18], data: &[u8]) -> i32 {
    for i in 0..18 {
        opl[i] = op_two_flags_from_bits(data[i] as u16);
    }
    0
}
