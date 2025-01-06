

struct Opl {
    is_op2: bool,
    op2_flags: [u16; 175],
    opl_gmtimbres: [OplVoice; 175],
    opl_gmtimbres_voice2: [OplVoice; 175],
}

struct OplVoice {
    notenum: u8,
    finetune: i8,
}

fn opl_load_op2_voice(voice: &mut OplVoice, buff: &[u8]) {
    // load voice data from buff
}

fn opl_loadbank_op2(opl: &mut Opl, data: &[u8]) -> Result<(), i32> {
    if data.len() < 8 + 36 * 175 {
        return Err(-3);
    }

    let buff = &data[8..];

    opl.is_op2 = true;

    for i in 0..175 {
        // OP2 instrument header
        opl.op2_flags[i] = u16::from(buff[0]) | (u16::from(buff[1]) << 8);

        let finetune = buff[2];
        let fixednote = buff[3];
        let buff = &buff[4..];

        // first voice
        opl_load_op2_voice(&mut opl.opl_gmtimbres[i], buff);
        opl.opl_gmtimbres[i].notenum = fixednote;
        let buff = &buff[16..];

        // second voice
        opl_load_op2_voice(&mut opl.opl_gmtimbres_voice2[i], buff);
        opl.opl_gmtimbres_voice2[i].notenum = fixednote;
        opl.opl_gmtimbres_voice2[i].finetune = (opl.opl_gmtimbres_voice2[i].finetune as u8)
            .wrapping_add(finetune as u8)
            .wrapping_sub(128) as i8
            as i8;
        let buff = &buff[16..];
    }

    Ok(())
}

