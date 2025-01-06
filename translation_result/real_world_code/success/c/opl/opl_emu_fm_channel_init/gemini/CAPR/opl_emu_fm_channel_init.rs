
struct opl_emu_fm_channel {
    m_choffs: u32,
    m_feedback: [u32; 2],
    m_feedback_in: u32,
    m_op: [Option<()>; 4],
    m_regs: Box<opl_emu_registers>,
}

struct opl_emu_registers;

fn opl_emu_fm_channel_init(fmch: &mut opl_emu_fm_channel, regs: Box<opl_emu_registers>, choffs: u32) {
    fmch.m_choffs = choffs;
    fmch.m_feedback = [0; 2];
    fmch.m_feedback_in = 0;
    fmch.m_op = [None, None, None, None];
    fmch.m_regs = regs;
}
