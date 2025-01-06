
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;

struct OplEmuRegistersOperatorMapping {
    chan: [u32; OPL_EMU_REGISTERS_CHANNELS],
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    opl_emu_bitfield(u32::from(regs.m_regdata[(offset + extra_offset) as usize]), start as i32, count as i32)
}

fn opl_emu_registers_operator_list(o1: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    u32::from(o1) | (u32::from(o2) << 8) | (u32::from(o3) << 16) | (u32::from(o4) << 24)
}

fn opl_emu_registers_fourop_enable(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x104, 0, 6, 0)
}

fn opl_emu_registers_operator_map(regs: &OplEmuRegisters, dest: &mut OplEmuRegistersOperatorMapping) {
    let fourop = opl_emu_registers_fourop_enable(regs);

    let mut set_chan = |chan_index: usize, condition: bool, list1: (u8, u8, u8, u8), list2: (u8, u8, u8, u8)| {
        dest.chan[chan_index] = if condition {
            opl_emu_registers_operator_list(list1.0, list1.1, list1.2, list1.3)
        } else {
            opl_emu_registers_operator_list(list2.0, list2.1, list2.2, list2.3)
        };
    };

    for i in 0..3 {
        set_chan(i, opl_emu_bitfield(fourop, i as i32, 1) != 0, (i as u8, i as u8 + 3, i as u8 + 6, i as u8 + 9), (i as u8, i as u8 + 3, 0xff, 0xff));
        set_chan(i + 3, false, (0xff, 0xff, 0xff, 0xff), (i as u8 + 6, i as u8 + 9, 0xff, 0xff));
    }

    for i in 6..9 {
        set_chan(i, true, (i as u8 + 6, i as u8 + 9, 0xff, 0xff), (0, 0, 0, 0)); // Always true for channels 6-8
    }

    for i in 9..12 {
        set_chan(i, opl_emu_bitfield(fourop, (i - 6) as i32, 1) != 0, (i as u8 + 9, i as u8 + 12, i as u8 + 15, i as u8 + 18), (i as u8 + 9, i as u8 + 12, 0xff, 0xff));
        set_chan(i + 3, false, (0xff, 0xff, 0xff, 0xff), (i as u8 + 15, i as u8 + 18, 0xff, 0xff));
    }

    for i in 15..18 {
        set_chan(i, true, (i as u8 + 15, i as u8 + 18, 0xff, 0xff), (0, 0, 0, 0)); // Always true for channels 15-17
    }
}
