

use std::fmt;

const OPL_EMU_REGISTERS_OPERATORS: u32 = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_STATUS_BUSY: u8 = 0;
const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_CHANNELS: u32 = 18;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x400;

#[derive(Copy, Clone, PartialEq, Eq)]
enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS as usize],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]; OPL_EMU_REGISTERS_WAVEFORMS as usize],
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; 6],
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
    m_regs: OplEmuRegisters,
    m_channel: [Option<Box<OplEmuFmChannel>>; OPL_EMU_REGISTERS_CHANNELS as usize],
    m_operator: [Option<Box<OplEmuFmOperator>>; OPL_EMU_REGISTERS_OPERATORS as usize],
}

impl OplEmu {
    fn opl_emu_bitfield(&self, value: u32, start: u32, length: u32) -> u32 {
        (value >> start) & ((1 << length) - 1)
    }
    fn opl_emu_registers_byte(
        &self,
        offset: u32,
        start: u32,
        count: u32,
        extra_offset: u32,
    ) -> u32 {
        self.opl_emu_bitfield(
            self.m_regs.m_regdata[offset as usize + extra_offset as usize] as u32,
            start,
            count,
        )
    }
    fn opl_emu_registers_status_mask(&self) -> u8 {
        (self.opl_emu_registers_byte(0x04, 0, 8, 0) & 0x78) as u8
    }
    fn opl_emu_set_reset_status(
        &mut self,
        set: u8,
        reset: u8,
    ) -> u8 {
        self.m_status = (self.m_status | set) & (!reset & !OPL_EMU_REGISTERS_STATUS_BUSY);
        //m_intf.opl_emu_sync_check_interrupts();
        self.m_status & !self.opl_emu_registers_status_mask()
    }
}

impl fmt::Debug for OplEmu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OplEmuRegisters {{ \
             m_lfo_am_counter: {:#04x}, \
             m_lfo_pm_counter: {:#04x}, \
             m_noise_lfsr: {:#08x}, \
             m_lfo_am: {:#02x}, \
             m_regdata: [...], \
             m_waveform: [...], \
             }}",
            self.m_regs.m_lfo_am_counter,
            self.m_regs.m_lfo_pm_counter,
            self.m_regs.m_noise_lfsr,
            self.m_regs.m_lfo_am,
        )
    }
}

