
#[derive(Clone)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8; 256]>,
    m_waveform: Box<[[u16; 256]; 3]>,
}

struct OplEmuFmOperator {
    // fields omitted for brevity
}

struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
    m_op: [Option<Box<OplEmuFmOperator>>; 4],
    m_regs: Box<OplEmuRegisters>,
}

fn opl_emu_fm_channel_init(fmch: &mut OplEmuFmChannel, regs: &OplEmuRegisters, choffs: u32) {
    fmch.m_choffs = choffs;
    fmch.m_feedback = [0, 0];
    fmch.m_feedback_in = 0;
    fmch.m_op = [None, None, None, None];
    fmch.m_regs = Box::new(regs.clone());
}
