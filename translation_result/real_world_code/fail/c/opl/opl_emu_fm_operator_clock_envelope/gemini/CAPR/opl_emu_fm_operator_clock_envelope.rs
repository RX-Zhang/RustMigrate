
const OPL_EMU_EG_ATTACK: u32 = 0;
const OPL_EMU_EG_DECAY: u32 = 1;
const OPL_EMU_EG_SUSTAIN: u32 = 2;

struct OplEmuFmOperator {
    m_env_state: u32,
    m_env_attenuation: u16,
    m_cache: Cache,
}

struct Cache {
    eg_sustain: u16,
    eg_rate: [u32; 4],
}

fn opl_emu_fm_operator_clock_envelope(fmop: &mut OplEmuFmOperator, env_counter: u32) {
    if fmop.m_env_state == OPL_EMU_EG_ATTACK && fmop.m_env_attenuation == 0 {
        fmop.m_env_state = OPL_EMU_EG_DECAY;
    }

    if fmop.m_env_state == OPL_EMU_EG_DECAY && u32::from(fmop.m_env_attenuation) >= u32::from(fmop.m_cache.eg_sustain) {
        fmop.m_env_state = OPL_EMU_EG_SUSTAIN;
    }

    let rate = fmop.m_cache.eg_rate[fmop.m_env_state as usize];
    let rate_shift = rate >> 2;
    let shifted_env_counter = env_counter << rate_shift;

    if opl_emu_bitfield(shifted_env_counter, 0, 11) != 0 {
        return;
    }

    let relevant_bits = opl_emu_bitfield(shifted_env_counter, (rate_shift <= 11) as i32, 3);
    let increment = opl_emu_attenuation_increment(rate, relevant_bits);

    if fmop.m_env_state == OPL_EMU_EG_ATTACK {
        if rate < 62 {
            fmop.m_env_attenuation = ((fmop.m_env_attenuation as u32).wrapping_add((!fmop.m_env_attenuation as u32 * increment) >> 4)) as u16;
        }
    } else {
        fmop.m_env_attenuation = ((fmop.m_env_attenuation as u32).wrapping_add(increment)) as u16;
        if fmop.m_env_attenuation >= 0x400 {
            fmop.m_env_attenuation = 0x3ff;
        }
    }
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let shift = start as u32;
    let mask = ((1 << (length as u32)) - 1) << shift;
    (value & mask) >> shift
}

fn opl_emu_attenuation_increment(rate: u32, index: u32) -> u32 {
    static S_INCREMENT_TABLE: [u32; 64] = [
        0x00000000, 0x00000000, 0x10101010, 0x10101010,  // 0-3    (0x00-0x03)
        0x10101010, 0x10101010, 0x11101110, 0x11101110,  // 4-7    (0x04-0x07)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 8-11   (0x08-0x0B)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 12-15  (0x0C-0x0F)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 16-19  (0x10-0x13)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 20-23  (0x14-0x17)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 24-27  (0x18-0x1B)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 28-31  (0x1C-0x1F)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 32-35  (0x20-0x23)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 36-39  (0x24-0x27)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 40-43  (0x28-0x2B)
        0x10101010, 0x10111010, 0x11101110, 0x11111110,  // 44-47  (0x2C-0x2F)
        0x11111111, 0x21112111, 0x21212121, 0x22212221,  // 48-51  (0x30-0x33)
        0x22222222, 0x42224222, 0x42424242, 0x44424442,  // 52-55  (0x34-0x37)
        0x44444444, 0x84448444, 0x84848484, 0x88848884,  // 56-59  (0x38-0x3B)
        0x88888888, 0x88888888, 0x88888888, 0x88888888,   // 60-63  (0x3C-0x3F)
    ];

    opl_emu_bitfield(S_INCREMENT_TABLE[rate as usize], 4 * (index as i32), 4)
}
