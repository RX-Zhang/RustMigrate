
use std::cmp::min;

#[derive(Clone, Copy)]
enum EnvelopeState {
    Attack,
    Decay,
    Sustain,
    Release,
}

struct Registers {
    lfo_am_counter: u16,
    lfo_pm_counter: u16,
    noise_lfsr: u32,
    lfo_am: u8,
    regdata: [u8; 0x200],
    waveform: [[u16; 0x400]; 8],
}

struct OpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; 6],
    eg_shift: u8,
}

struct FmOperator {
    choffs: u32,
    opoffs: u32,
    phase: u32,
    env_attenuation: u16,
    env_state: EnvelopeState,
    key_state: u8,
    keyon_live: u8,
    cache: OpdataCache,
    regs: Box<Registers>,
}

fn bitfield(value: u32, start: usize, length: usize) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn registers_byte(regs: &Registers, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    bitfield(regs.regdata[offset as usize + extra_offset as usize] as u32, start as usize, count as usize)
}

fn registers_op_lfo_am_enable(regs: &Registers, opoffs: u32) -> u32 {
    registers_byte(regs, 0x20, 7, 1, opoffs)
}

fn fm_operator_envelope_attenuation(fmop: &FmOperator, am_offset: u32) -> u32 {
    let mut result = fmop.env_attenuation >> fmop.cache.eg_shift as u32;

    // add in LFO AM modulation
    if registers_op_lfo_am_enable(fmop.regs.as_ref(), fmop.opoffs) != 0 {
        result = result.wrapping_add(am_offset.try_into().unwrap());
    }

    // add in total level and KSL from the cache
    result = result.wrapping_add(fmop.cache.total_level.try_into().unwrap());

    // clamp to max, apply shift, and return
    min(result.into(), 0x3ff)
}
