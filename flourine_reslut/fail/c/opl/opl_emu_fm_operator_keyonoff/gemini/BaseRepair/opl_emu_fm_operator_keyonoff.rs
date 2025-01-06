
use std::boxed::Box;

#[derive(Clone, Copy)]
enum EnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

#[derive(Clone, Copy)]
enum KeyOnType {
    Normal = 0,
    Rhythm = 1,
    Csm = 2,
}

struct Registers {
    lfo_am_counter: u16,
    lfo_pm_counter: u16,
    noise_lfsr: u32,
    lfo_am: u8,
    regdata: Box<[u8; 0x200]>,
    waveform: Box<[[u16; 0x400]; 8]>,
}

struct OpDataCache {
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
    cache: OpDataCache,
    regs: Box<Registers>,
}

fn bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn keyonoff(fmop: &mut FmOperator, on: u32, typ: KeyOnType) {
    fmop.keyon_live = (fmop.keyon_live & !(1 << typ as usize)) | ((bitfield(on, 0, 1) as u8) << typ as usize);
}
