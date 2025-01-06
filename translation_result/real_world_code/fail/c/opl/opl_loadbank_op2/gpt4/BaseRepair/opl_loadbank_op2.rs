
enum Op2Flags {
    FixedPitch,
    Unused,
    DoubleVoice,
}

fn u16from_le_bytes(bytes: [u8; 2]) -> u16 {
    u16::from_le_bytes(bytes)
}

fn opl_load_op2_voice(voice: &mut Voice, data: &[u8]) {
    // Implementation for loading an OP2 voice
}

struct Opl {
    is_op2: i32,
    op2_flags: [Op2Flags; 175],
    opl_gmtimbres: [Voice; 175],
    opl_gmtimbres_voice2: [Voice; 175],
}

struct Voice {
    notenum: u8,
    finetune: i8,
    // Other fields...
}

fn opl_loadbank_op2(opl: &mut Opl, data: &[u8]) -> i32 {
    if data.len() < 8 + 36 * 175 {
        return -3;
    }
    let mut buff = &data[8..];

    opl.is_op2 = 1;
    for i in 0..175 {
        let flag_value = u16from_le_bytes([buff[0], buff[1]]);
        opl.op2_flags[i] = if (flag_value & (1 << 0)) != 0 {
            Op2Flags::FixedPitch
        } else if (flag_value & (1 << 1)) != 0 {
            Op2Flags::Unused
        } else if (flag_value & (1 << 2)) != 0 {
            Op2Flags::DoubleVoice
        } else {
            continue;
        };

        let finetune = buff[2];
        let fixednote = buff[3];
        buff = &buff[4..];

        opl_load_op2_voice(&mut opl.opl_gmtimbres[i], &buff[..16]);
        opl.opl_gmtimbres[i].notenum = fixednote;
        buff = &buff[16..];

        opl_load_op2_voice(&mut opl.opl_gmtimbres_voice2[i], &buff[..16]);
        opl.opl_gmtimbres_voice2[i].notenum = fixednote;
        opl.opl_gmtimbres_voice2[i].finetune += finetune.wrapping_sub(128) as i8;
        buff = &buff[16..];
    }
    0
}
