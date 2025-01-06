
use std::alloc::{alloc_zeroed, Layout};
use std::ptr::write;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>,
}

impl OplEmuRegisters {
    fn new() -> Self {
        let layout = Layout::array::<[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]>(OPL_EMU_REGISTERS_WAVEFORMS).unwrap();
        let ptr = unsafe { alloc_zeroed(layout) as *mut [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS] };
        let m_waveform = unsafe { Box::from_raw(ptr) };

        Self {
            m_lfo_am_counter: 0,
            m_lfo_pm_counter: 0,
            m_noise_lfsr: 0,
            m_lfo_am: 0,
            m_regdata: [0; OPL_EMU_REGISTERS_REGISTERS],
            m_waveform,
        }
    }

    fn opl_emu_registers_noise_state(&self) -> u32 {
        self.m_noise_lfsr >> 23
    }
}

fn main() {
    let regs = OplEmuRegisters::new();
    println!("{}", regs.opl_emu_registers_noise_state());
}
