



use std::boxed::Box;

#[derive(Copy, Clone)]
pub enum OplEmuEnvelopeState {
    OplEmuEgAttack = 1,
    OplEmuEgDecay = 2,
    OplEmuEgSustain = 3,
    OplEmuEgRelease = 4,
    OplEmuEgStates = 6,
}

#[derive(Copy, Clone)]
pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: [u8; 0x200],
    pub m_waveform: [[u16; 0x400]; 8],
}

#[derive(Copy, Clone)]
pub struct OplEmuOpdataCache {
    pub phase_step: u32,
    pub total_level: u32,
    pub block_freq: u32,
    pub detune: i32,
    pub multiple: u32,
    pub eg_sustain: u32,
    pub eg_rate: [u8; 6],
    pub eg_shift: u8,
}

#[derive(Copy, Clone)]
pub struct OplEmuFmOperator {
    pub m_choffs: u32,
    pub m_opoffs: u32,
    pub m_phase: u32,
    pub m_env_attenuation: u16,
    pub m_env_state: OplEmuEnvelopeState,
    pub m_key_state: u8,
    pub m_keyon_live: u8,
    pub m_cache: OplEmuOpdataCache,
}

#[derive(Copy, Clone)]
pub struct OplEmuFmChannel {
    pub m_choffs: u32,
    pub m_feedback: [i16; 2],
    pub m_feedback_in: i16,
}

pub struct OplEmu {
    pub m_env_counter: u32,
    pub m_status: u8,
    pub m_timer_running: [u8; 2],
    pub m_active_channels: u32,
    pub m_modified_channels: u32,
    pub m_prepare_count: u32,
    pub m_regs: Box<OplEmuRegisters>,
    pub m_channel: Vec<Box<OplEmuFmChannel>>,
    pub m_operator: Vec<Box<OplEmuFmOperator>>,
}

pub fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start as u32) & ((1 << length as u32) - 1)
}

pub fn opl_emu_registers_byte(
    regs: &Box<OplEmuRegisters>,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    opl_emu_bitfield(
        regs.m_regdata[offset as usize + extra_offset as usize] as u32,
        start as i32,
        count as i32,
    )
}

pub fn opl_emu_registers_timer_b_value(regs: &Box<OplEmuRegisters>) -> u32 {
    opl_emu_registers_byte(regs, 0x03, 0, 8, 0)
}

pub fn opl_emu_registers_timer_a_value(regs: &Box<OplEmuRegisters>) -> u32 {
    opl_emu_registers_byte(regs, 0x02, 0, 8, 0) * 4
}

pub fn opl_emu_update_timer(emu: &mut Box<OplEmu>, tnum: u32, enable: u32) {
    if enable != 0 && emu.m_timer_running[tnum as usize] == 0 {
        let period = if tnum == 0 {
            1024 - opl_emu_registers_timer_a_value(&emu.m_regs)
        } else {
            16 * (256 - opl_emu_registers_timer_b_value(&emu.m_regs))
        };
        emu.m_timer_running[tnum as usize] = 1;
    } else if enable == 0 {
        emu.m_timer_running[tnum as usize] = 0;
    }
}



