
use std::ops::BitAnd;
use std::ops::Shr;
use std::boxed::Box;

fn opl_emu_bitfield(value: u32, start: u32, count: u32) -> u32 {
    (value.bitand((((1 << (start + count)) - 1) << (32 - start - count)) >> (32 - count))) >> start
}

fn opl_emu_registers_byte(
    regs: &mut Box<[u32]>,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    let value = regs[offset as usize + extra_offset as usize] as u32;
    opl_emu_bitfield(value, start, count)
}

fn opl_emu_registers_timer_b_value(regs: &mut Box<[u32]>) -> u32 {
    opl_emu_registers_byte(regs, 0x03, 0, 8, 0)
}

fn opl_emu_registers_timer_a_value(regs: &mut Box<[u32]>) -> u32 {
    opl_emu_registers_byte(regs, 0x02, 0, 8, 0).wrapping_shr(3)
}

struct OplEmu {
    m_timer_running: [bool; 2],
    m_regs: Box<[u32]>,
}

fn opl_emu_update_timer(emu: &mut OplEmu, tnum: u32, enable: u32) {
    if enable != 0 && emu.m_timer_running[tnum as usize] == false {
        let period =
            if tnum == 0 {
                (1024 - opl_emu_registers_timer_a_value(&mut emu.m_regs))
                    .wrapping_add(1)
                    .wrapping_shr(1)
            } else {
                16 * (256 - opl_emu_registers_timer_b_value(&mut emu.m_regs))
            };
        emu.m_timer_running[tnum as usize] = true;
    } else if enable == 0 {
        emu.m_timer_running[tnum as usize] = false;
    }
}
