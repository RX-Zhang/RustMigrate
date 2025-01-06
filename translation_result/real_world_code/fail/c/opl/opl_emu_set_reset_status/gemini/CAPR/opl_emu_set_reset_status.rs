
#![allow(non_camel_case_types, non_snake_case)]


const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_STATUS_BUSY: u8 = 0;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;
const OPL_EMU_EG_STATES: usize = 6;

#[derive(Debug, Clone, Copy)]
enum opl_emu_envelope_state {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
    OPL_EMU_EG_STATES = 6,
}

struct opl_emu_registers {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]
}

struct opl_emu_opdata_cache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; OPL_EMU_EG_STATES],
    eg_shift: u8,
}

struct opl_emu_fm_operator {
    m_choffs: u32,
    m_opoffs: u32,
    m_phase: u32,
    m_env_attenuation: u16,
    m_env_state: opl_emu_envelope_state,
    m_key_state: u8,
    m_keyon_live: u8,
    m_cache: opl_emu_opdata_cache,
    m_regs: Box<opl_emu_registers>,
}

struct opl_emu_fm_channel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
}

struct opl_emu_t {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: [u8; 2],
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: Box<opl_emu_registers>,
    m_channel: [opl_emu_fm_channel; OPL_EMU_REGISTERS_CHANNELS],
    m_operator: [opl_emu_fm_operator; OPL_EMU_REGISTERS_OPERATORS],
}

fn opl_emu_bitfield(value: u32, start: usize, length: usize) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &opl_emu_registers, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    opl_emu_bitfield(regs.m_regdata[(offset + extra_offset) as usize] as u32, start as usize, count as usize)
}

fn opl_emu_registers_status_mask(regs: &opl_emu_registers) -> u32 {
    opl_emu_registers_byte(regs, 0x04, 0, 8, 0) & 0x78
}

fn opl_emu_set_reset_status(emu: &mut opl_emu_t, set: u8, reset: u8) -> u8 {
    emu.m_status = (emu.m_status | set) & !(reset | OPL_EMU_REGISTERS_STATUS_BUSY);
    // m_intf.opl_emu_sync_check_interrupts();
    emu.m_status & !opl_emu_registers_status_mask(&emu.m_regs) as u8
}
