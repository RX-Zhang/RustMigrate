
use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd)]
enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
    OPL_EMU_EG_STATES = 6,
}

struct OplEmuFmOperator {
    m_env_state: OplEmuEnvelopeState,
    m_phase: u32,
    m_env_attenuation: u32,
    m_cache: OplEmuFmOperatorCache,
}

struct OplEmuFmOperatorCache {
    eg_rate: [u8; 6],
}

fn opl_emu_fm_operator_start_release(fmop: &mut OplEmuFmOperator) {
    match fmop.m_env_state.partial_cmp(&OplEmuEnvelopeState::OPL_EMU_EG_RELEASE) {
        Some(Ordering::Less) | None => {
            fmop.m_env_state = OplEmuEnvelopeState::OPL_EMU_EG_RELEASE;
        }
        _ => {}
    }
}

fn opl_emu_fm_operator_start_attack(fmop: &mut OplEmuFmOperator) {
    if fmop.m_env_state != OplEmuEnvelopeState::OPL_EMU_EG_ATTACK {
        fmop.m_env_state = OplEmuEnvelopeState::OPL_EMU_EG_ATTACK;
        fmop.m_phase = 0;
        if fmop.m_cache.eg_rate[0] >= 62 {
            fmop.m_env_attenuation = 0;
        }
    }
}

