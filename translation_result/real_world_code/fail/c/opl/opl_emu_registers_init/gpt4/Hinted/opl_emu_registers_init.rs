
struct OplEmuRegisters {
    m_lfo_am_counter: i32,
 m_lfo_pm_counter: i32,
    m_noise_lfsr: i32,
    m_lfo_am: i32,
    m_regdata: [i32; 256],
}

fn opl_emu_registers_init() -> Box<OplEmuRegisters> {
    let regs = OplEmuRegisters {
        m_lfo_am_counter: 0,
        m_lfo_pm_counter: 0,
        m_noise_lfsr: 1,
        m_lfo_am: 0,
        m_regdata: [0; 256],
    };
    Box::new(regs)
}
