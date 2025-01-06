
use std::ops::{Shr, Shl, BitAnd, BitOr, BitXor, Sub, Add, Rem};

const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x400;

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_lfo_am_divider: u16,
    m_lfo_pm_divider: u16,
    m_lfo_am_shift: u16,
    m_lfo_pm_shift: u16,
    m_amp_modulation: u16,
    m_phase_modulation: u16,
    m_feedback: u16,
    m_output: u16,
    m_waveform_select: u16,
    m_frequency_divider: u16,
    m_frequency_shift: u16,
    m_frequency_shift_fine: u16,
    m_frequency_counter: u16,
    m_frequency_counter_reload: u16,
}

fn opl_emu_register_write(registers: &mut OplEmuRegisters, address: u32, value: u16) {
    let reg_offset = address as usize % std::mem::size_of::<OplEmuRegisters>() / std::mem::size_of::<u16>();
    let reg_val = (value as u32) << (16 * reg_offset);
    let reg_mask = (!0u32) << (16 * reg_offset);
    let reg_ptr = unsafe { std::mem::transmute::<&mut OplEmuRegisters, *mut u16>(registers) };
    unsafe {
        *reg_ptr.add(reg_offset) = (*reg_ptr.add(reg_offset)) & !(reg_mask as u16) | (reg_val as u16) & (reg_mask as u16);
    }
}
