
use std::boxed::Box;

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Clone, Copy)]
enum OplEmuEnvelopeState {
    OplEmuEgAttack = 1,
    OplEmuEgDecay = 2,
    OplEmuEgSustain = 3,
    OplEmuEgRelease = 4,
    OplEmuEgStates = 6,
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; OplEmuEnvelopeState::OplEmuEgStates as usize],
    eg_shift: u8,
}

struct OplEmuFmOperator {
    m_choffs: u32,
    m_opoffs: u32,
    m_phase: u32,
    m_env_attenuation: u16,
    m_env_state: OplEmuEnvelopeState,
    m_key_state: u8,
    m_keyon_live: u8,
    m_cache: OplEmuOpdataCache,
}

#[derive(Clone, Copy)]
struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
}

struct OplEmu {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: [u8; 2],
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: Box<OplEmuRegisters>,
    m_channel: Box<[OplEmuFmChannel; OPL_EMU_REGISTERS_CHANNELS]>,
    m_operator: Box<[OplEmuFmOperator; OPL_EMU_REGISTERS_OPERATORS]>,
}

impl OplEmu {
    fn new() -> Self {
        Self {
            m_env_counter: 0,
            m_status: 0,
            m_timer_running: [0; 2],
            m_active_channels: 0,
            m_modified_channels: 0,
            m_prepare_count: 0,
            m_regs: Box::new(OplEmuRegisters {
                m_lfo_am_counter: 0,
                m_lfo_pm_counter: 0,
                m_noise_lfsr: 0,
                m_lfo_am: 0,
                m_regdata: [0; OPL_EMU_REGISTERS_REGISTERS],
                m_waveform: [[0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
            }),
            m_channel: Box::new([(); OPL_EMU_REGISTERS_CHANNELS].map(|_| OplEmuFmChannel {
                m_choffs: 0,
                m_feedback: [0; 2],
                m_feedback_in: 0,
            })),
            m_operator: Box::new([(); OPL_EMU_REGISTERS_OPERATORS].map(|_| OplEmuFmOperator {
                m_choffs: 0,
                m_opoffs: 0,
                m_phase: 0,
                m_env_attenuation: 0,
                m_env_state: OplEmuEnvelopeState::OplEmuEgAttack,
                m_key_state: 0,
                m_keyon_live: 0,
                m_cache: OplEmuOpdataCache {
                    phase_step: 0,
                    total_level: 0,
                    block_freq: 0,
                    detune: 0,
                    multiple: 0,
                    eg_sustain: 0,
                    eg_rate: [0; OplEmuEnvelopeState::OplEmuEgStates as usize],
                    eg_shift: 0,
                },
            })),
        }
    }
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    opl_emu_bitfield(regs.m_regdata[(offset + extra_offset) as usize] as u32, start as i32, count as i32)
}

fn opl_emu_registers_timer_b_value(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x03, 0, 8, 0)
}

fn opl_emu_registers_timer_a_value(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x02, 0, 8, 0) * 4
}

fn opl_emu_update_timer(emu: &mut OplEmu, tnum: u32, enable: u32) {
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
