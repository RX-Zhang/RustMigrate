

use std::mem;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,            // LFO AM counter
    m_lfo_pm_counter: u16,            // LFO PM counter
    m_noise_lfsr: u32,                // noise LFSR state
    m_lfo_am: u8,                     // current LFO AM value
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],         // register data
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS], // waveforms
}

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: usize, start: u32, count: u32, extra_offset: usize) -> u32 {
    opl_emu_bitfield(regs.m_regdata[offset + extra_offset] as u32, start, count)
}

fn opl_emu_opl_clock_noise_and_lfo(
    noise_lfsr: &mut u32,
    lfo_am_counter: &mut u16,
    lfo_pm_counter: &mut u16,
    lfo_am: &mut u8,
    am_depth: u32,
    pm_depth: u32,
) -> i32 {
    // OPL has a 23-bit noise generator for the rhythm section, running at
    // a constant rate, used only for percussion input
    let mut noise_lfsr_local = *noise_lfsr;
    noise_lfsr_local = (noise_lfsr_local << 1) | ((noise_lfsr_local >> 23) ^ (noise_lfsr_local >> 9) ^ (noise_lfsr_local >> 8) ^ (noise_lfsr_local & 1));
    *noise_lfsr = noise_lfsr_local;

    // OPL has two fixed-frequency LFOs, one for AM, one for PM

    // the AM LFO has 210*64 steps; at a nominal 50kHz output,
    // this equates to a period of 50000/(210*64) = 3.72Hz
    let am_counter_local = lfo_am_counter.wrapping_add(1);
    if am_counter_local >= 210 * 64 - 1 {
        *lfo_am_counter = 0;
    }

    // low 8 bits are fractional; depth 0 is divided by 2, while depth 1 is times 2
    let shift = 9 - 2 * am_depth;

    // AM value is the upper bits of the value, inverted across the midpoint
    // to produce a triangle
    *lfo_am = if am_counter_local < 105 * 64 {
        (am_counter_local >> shift) as u8
    } else {
        (((210 * 64 + 63) - am_counter_local) >> shift) as u8
    };

    // the PM LFO has 8192 steps, or a nominal period of 6.1Hz
    let pm_counter_local = lfo_pm_counter.wrapping_add(1);

    // PM LFO is broken into 8 chunks, each lasting 1024 steps; the PM value
    // depends on the upper bits of FNUM, so this value is a fraction and
    // sign to apply to that value, as a 1.3 value
    let pm_scale = [8, 4, 0, -4, -8, -4, 0, 4];
    (pm_scale[((lfo_pm_counter.wrapping_add(1) >> 10) & 7) as usize] >> (pm_depth ^ 1)) as i32
}

fn opl_emu_registers_lfo_pm_depth(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0xbd, 6, 1, 0)
}

fn opl_emu_registers_lfo_am_depth(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0xbd, 7, 1, 0)
}

fn opl_emu_registers_clock_noise_and_lfo(regs: &mut OplEmuRegisters) -> i32 {
    let lfo_am_depth = opl_emu_registers_lfo_am_depth(regs);
    let lfo_pm_depth = opl_emu_registers_lfo_pm_depth(regs);
    opl_emu_opl_clock_noise_and_lfo(
        &mut regs.m_noise_lfsr,
        &mut regs.m_lfo_am_counter,
        &mut regs.m_lfo_pm_counter,
        &mut regs.m_lfo_am,
        lfo_am_depth,
        lfo_pm_depth,
    )
}

