

use std::mem;

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_ALL_CHANNELS: u32 = (1 << OPL_EMU_REGISTERS_CHANNELS) - 1;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;
const OPL_EMU_EG_STATES: usize = 6;

#[derive(Copy, Clone, PartialEq)]
enum OplEmuEnvelopeState {
    OplEmuEgAttack = 1,
    OplEmuEgDecay = 2,
    OplEmuEgSustain = 3,
    OplEmuEgRelease = 4,
    OplEmuEgStates = 6,
}

#[derive(Clone)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS]>,
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>,
}

struct OplEmuRegistersOperatorMapping {
    chan: [u32; OPL_EMU_REGISTERS_CHANNELS],
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; OPL_EMU_EG_STATES],
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
    m_regs: Box<OplEmuRegisters>,
}

struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
    m_op: [Option<Box<OplEmuFmOperator>>; 4],
    m_regs: Box<OplEmuRegisters>,
}

struct OplEmuT {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: [u8; 2],
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: [OplEmuFmChannel; OPL_EMU_REGISTERS_CHANNELS],
    m_operator: [Option<Box<OplEmuFmOperator>>; OPL_EMU_REGISTERS_OPERATORS],
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(
    regs: &OplEmuRegisters,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    opl_emu_bitfield(
        regs.m_regdata[offset as usize + extra_offset as usize] as u32,
        start as i32,
        count as i32,
    )
}

fn opl_emu_fm_operator_set_choffs(fmop: &mut OplEmuFmOperator, choffs: u32) {
    fmop.m_choffs = choffs;
}

fn opl_emu_registers_operator_list(o1: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    (o1 as u32) | ((o2 as u32) << 8) | ((o3 as u32) << 16) | ((o4 as u32) << 24)
}

fn opl_emu_registers_fourop_enable(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x104, 0, 6, 0)
}

fn opl_emu_fm_channel_assign(
    fmch: &mut OplEmuFmChannel,
    index: u32,
    op: Option<Box<OplEmuFmOperator>>,
) {
    let mut op_mut = op;
    fmch.m_op[index as usize] = op_mut.take();
    if let Some(op) = op_mut.as_mut() {
        opl_emu_fm_operator_set_choffs(op, fmch.m_choffs);
    }
}

fn opl_emu_registers_operator_map(
    regs: &OplEmuRegisters,
    dest: &mut OplEmuRegistersOperatorMapping,
) {
    let fourop = opl_emu_registers_fourop_enable(regs);

    dest.chan[0] = if opl_emu_bitfield(fourop, 0, 1) != 0 {
        opl_emu_registers_operator_list(0, 3, 6, 9)
    } else {
        opl_emu_registers_operator_list(0, 3, 0xff, 0xff)
    };
    dest.chan[1] = if opl_emu_bitfield(fourop, 1, 1) != 0 {
        opl_emu_registers_operator_list(1, 4, 7, 10)
    } else {
        opl_emu_registers_operator_list(1, 4, 0xff, 0xff)
    };
    dest.chan[2] = if opl_emu_bitfield(fourop, 2, 1) != 0 {
        opl_emu_registers_operator_list(2, 5, 8, 11)
    } else {
        opl_emu_registers_operator_list(2, 5, 0xff, 0xff)
    };
    dest.chan[3] = if opl_emu_bitfield(fourop, 0, 1) != 0 {
        opl_emu_registers_operator_list(0xff, 0xff, 0xff, 0xff)
    } else {
        opl_emu_registers_operator_list(6, 9, 0xff, 0xff)
    };
    dest.chan[4] = if opl_emu_bitfield(fourop, 1, 1) != 0 {
        opl_emu_registers_operator_list(0xff, 0xff, 0xff, 0xff)
    } else {
        opl_emu_registers_operator_list(7, 10, 0xff, 0xff)
    };
    dest.chan[5] = if opl_emu_bitfield(fourop, 2, 1) != 0 {
        opl_emu_registers_operator_list(0xff, 0xff, 0xff, 0xff)
    } else {
        opl_emu_registers_operator_list(8, 11, 0xff, 0xff)
    };
    dest.chan[6] = opl_emu_registers_operator_list(12, 15, 0xff, 0xff);
    dest.chan[7] = opl_emu_registers_operator_list(13, 16, 0xff, 0xff);
    dest.chan[8] = opl_emu_registers_operator_list(14, 17, 0xff, 0xff);

    dest.chan[9] = if opl_emu_bitfield(fourop, 3, 1) != 0 {
        opl_emu_registers_operator_list(18, 21, 24, 27)
    } else {
        opl_emu_registers_operator_list(18, 21, 0xff, 0xff)
    };
    dest.chan[10] = if opl_emu_bitfield(fourop, 4, 1) != 0 {
        opl_emu_registers_operator_list(19, 22, 25, 28)
    } else {
        opl_emu_registers_operator_list(19, 22, 0xff, 0xff)
    };
    dest.chan[11] = if opl_emu_bitfield(fourop, 5, 1) != 0 {
        opl_emu_registers_operator_list(20, 23, 26, 29)
    } else {
        opl_emu_registers_operator_list(20, 23, 0xff, 0xff)
    };
    dest.chan[12] = if opl_emu_bitfield(fourop, 3, 1) != 0 {
        opl_emu_registers_operator_list(0xff, 0xff, 0xff, 0xff)
    } else {
        opl_emu_registers_operator_list(24, 27, 0xff, 0xff)
    };
    dest.chan[13] = if opl_emu_bitfield(fourop, 4, 1) != 0 {
        opl_emu_registers_operator_list(0xff, 0xff, 0xff, 0xff)
    } else {
        opl_emu_registers_operator_list(25, 28, 0xff, 0xff)
    };
    dest.chan[14] = if opl_emu_bitfield(fourop, 5, 1) != 0 {
        opl_emu_registers_operator_list(0xff, 0xff, 0xff, 0xff)
    } else {
        opl_emu_registers_operator_list(26, 29, 0xff, 0xff)
    };
    dest.chan[15] = opl_emu_registers_operator_list(30, 33, 0xff, 0xff);
    dest.chan[16] = opl_emu_registers_operator_list(31, 34, 0xff, 0xff);
    dest.chan[17] = opl_emu_registers_operator_list(32, 35, 0xff, 0xff);
}

fn opl_emu_abs_sin_attenuation(input: u32) -> u16 {
    static S_SIN_TABLE: [u16; 256] = [
        0x859, 0x6c3, 0x607, 0x58b, 0x52e, 0x4e4, 0x4a6, 0x471, 0x443, 0x41a, 0x3f5, 0x3d3, 0x3b5,
        0x398, 0x37e, 0x365, 0x34e, 0x339, 0x324, 0x311, 0x2ff, 0x2ed, 0x2dc, 0x2cd, 0x2bd, 0x2af,
        0x2a0, 0x293, 0x286, 0x279, 0x26d, 0x261, 0x256, 0x24b, 0x240, 0x236, 0x22c, 0x222, 0x218,
        0x20f, 0x206, 0x1fd, 0x1f5, 0x1ec, 0x1e4, 0x1dc, 0x1d4, 0x1cd, 0x1c5, 0x1be, 0x1b7, 0x1b0,
        0x1a9, 0x1a2, 0x19b, 0x195, 0x18f, 0x188, 0x182, 0x17c, 0x177, 0x171, 0x16b, 0x166, 0x160,
        0x15b, 0x155, 0x150, 0x14b, 0x146, 0x141, 0x13c, 0x137, 0x133, 0x12e, 0x129, 0x125, 0x121,
        0x11c, 0x118, 0x114, 0x10f, 0x10b, 0x107, 0x103, 0x0ff, 0x0fb, 0x0f8, 0x0f4, 0x0f0, 0x0ec,
        0x0e9, 0x0e5, 0x0e2, 0x0de, 0x0db, 0x0d7, 0x0d4, 0x0d1, 0x0cd, 0x0ca, 0x0c7, 0x0c4, 0x0c1,
        0x0be, 0x0bb, 0x0b8, 0x0b5, 0x0b2, 0x0af, 0x0ac, 0x0a9, 0x0a7, 0x0a4, 0x0a1, 0x09f, 0x09c,
        0x099, 0x097, 0x094, 0x092, 0x08f, 0x08d, 0x08a, 0x088, 0x086, 0x083, 0x081, 0x07f, 0x07d,
        0x07a, 0x078, 0x076, 0x074, 0x072, 0x070, 0x06e, 0x06c, 0x06a, 0x068, 0x066, 0x064, 0x062,
        0x060, 0x05e, 0x05c, 0x05b, 0x059, 0x057, 0x055, 0x053, 0x052, 0x050, 0x04e, 0x04d, 0x04b,
        0x04a, 0x048, 0x046, 0x045, 0x043, 0x042, 0x040, 0x03f, 0x03e, 0x03c, 0x03b, 0x039, 0x038,
        0x037, 0x035, 0x034, 0x033, 0x031, 0x030, 0x02f, 0x02e, 0x02d, 0x02b, 0x02a, 0x029, 0x028,
        0x027, 0x026, 0x025, 0x024, 0x023, 0x022, 0x021, 0x020, 0x01f, 0x01e, 0x01d, 0x01c, 0x01b,
        0x01a, 0x019, 0x018, 0x017, 0x017, 0x016, 0x015, 0x014, 0x014, 0x013, 0x012, 0x011, 0x011,
        0x010, 0x00f, 0x00f, 0x00e, 0x00d, 0x00d, 0x00c, 0x00c, 0x00b, 0x00a, 0x00a, 0x009, 0x009,
        0x008, 0x008, 0x007, 0x007, 0x007, 0x006, 0x006, 0x005, 0x005, 0x005, 0x004, 0x004, 0x004,
        0x003, 0x003, 0x003, 0x002, 0x002, 0x002, 0x002, 0x001, 0x001, 0x001, 0x001, 0x001, 0x001,
        0x001, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000,
    ];

    if opl_emu_bitfield(input, 8, 1) != 0 {
        S_SIN_TABLE[(!input & 0xff) as usize]
    } else {
        S_SIN_TABLE[input as usize]
    }
}

fn opl_emu_assign_operators(emu: &mut OplEmuT) {
    let mut map = OplEmuRegistersOperatorMapping {
        chan: [0; OPL_EMU_REGISTERS_CHANNELS],
    };
    opl_emu_registers_operator_map(&emu.m_regs, &mut map);

    for chnum in 0..OPL_EMU_REGISTERS_CHANNELS {
        for index in 0..4 {
            let opnum = opl_emu_bitfield(map.chan[chnum], (index * 8) as i32, 8);
            let op = if opnum == 0xff {
                None
            } else {
                let mut new_op = OplEmuFmOperator {
                    m_choffs: 0,
                    m_opoffs: 0,
                    m_phase: 0,
                    m_env_attenuation: 0x3ff,
                    m_env_state: OplEmuEnvelopeState::OplEmuEgRelease,
                    m_key_state: 0,
                    m_keyon_live: 0,
                    m_cache: OplEmuOpdataCache {
                        phase_step: 0,
                        total_level: 0,
                        block_freq: 0,
                        detune: 0,
                        multiple: 0,
                        eg_sustain: 0,
                        eg_rate: [0; OPL_EMU_EG_STATES],
                        eg_shift: 0,
                    },
                    m_regs: Box::new(emu.m_regs.clone()),
                };
                opl_emu_fm_operator_init(&mut new_op, &emu.m_regs, opnum);
                Some(Box::new(new_op))
            };
            opl_emu_fm_channel_assign(&mut emu.m_channel[chnum], index, op);
        }
    }
}

fn opl_emu_fm_operator_init(fmop: &mut OplEmuFmOperator, regs: &OplEmuRegisters, opoffs: u32) {
    fmop.m_choffs = 0;
    fmop.m_opoffs = opoffs;
    fmop.m_phase = 0;
    fmop.m_env_attenuation = 0x3ff;
    fmop.m_env_state = OplEmuEnvelopeState::OplEmuEgRelease;
    fmop.m_key_state = 0;
    fmop.m_keyon_live = 0;
    fmop.m_cache = OplEmuOpdataCache {
        phase_step: 0,
        total_level: 0,
        block_freq: 0,
        detune: 0,
        multiple: 0,
        eg_sustain: 0,
        eg_rate: [0; OPL_EMU_EG_STATES],
        eg_shift: 0,
    };
    fmop.m_regs = Box::new(regs.clone());
}

fn opl_emu_registers_operator_offset(opnum: u32) -> u32 {
    (opnum % 18) + 2 * ((opnum % 18) / 6) + 0x100 * (opnum / 18)
}

fn opl_emu_fm_channel_init(fmch: &mut OplEmuFmChannel, regs: &OplEmuRegisters, choffs: u32) {
    fmch.m_choffs = choffs;
    fmch.m_feedback = [0, 0];
    fmch.m_feedback_in = 0;
    fmch.m_op = [None, None, None, None];
    fmch.m_regs = Box::new(regs.clone());
}

fn opl_emu_registers_channel_offset(chnum: u32) -> u32 {
    (chnum % 9) + 0x100 * (chnum / 9)
}

fn opl_emu_registers_init(regs: &mut OplEmuRegisters) {
    regs.m_lfo_am_counter = 0;
    regs.m_lfo_pm_counter = 0;
    regs.m_noise_lfsr = 1;
    regs.m_lfo_am = 0;
    regs.m_regdata = Box::new([0; OPL_EMU_REGISTERS_REGISTERS]);
    regs.m_waveform = Box::new([[0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]);

    for index in 0..OPL_EMU_REGISTERS_WAVEFORM_LENGTH {
        regs.m_waveform[0][index] =
            opl_emu_abs_sin_attenuation(index as u32) | ((opl_emu_bitfield(index as u32, 9, 1) << 15) as u16);
    }

    if OPL_EMU_REGISTERS_WAVEFORMS >= 4 {
        let zeroval = regs.m_waveform[0][0];
        for index in 0..OPL_EMU_REGISTERS_WAVEFORM_LENGTH {
            regs.m_waveform[1][index] = if opl_emu_bitfield(index as u32, 9, 1) != 0 {
                zeroval
            } else {
                regs.m_waveform[0][index]
            };
            regs.m_waveform[2][index] = regs.m_waveform[0][index] & 0x7fff;
            regs.m_waveform[3][index] = if opl_emu_bitfield(index as u32, 8, 1) != 0 {
                zeroval
            } else {
                regs.m_waveform[0][index] & 0x7fff
            };
            if OPL_EMU_REGISTERS_WAVEFORMS >= 8 {
                regs.m_waveform[4][index] = if opl_emu_bitfield(index as u32, 9, 1) != 0 {
                    zeroval
                } else {
                    regs.m_waveform[0][(index * 2) as usize]
                };
                regs.m_waveform[5][index] = if opl_emu_bitfield(index as u32, 9, 1) != 0 {
                    zeroval
                } else {
                    regs.m_waveform[0][((index * 2) & 0x1ff) as usize]
                };
                regs.m_waveform[6][index] = (opl_emu_bitfield(index as u32, 9, 1) << 15) as u16;
                regs.m_waveform[7][index] = (if opl_emu_bitfield(index as u32, 9, 1) != 0 {
                    (index ^ 0x13ff).wrapping_add(1)
                } else {
                    index
                } << 3) as u16;
            }
        }
    }
}

fn opl_emu_init(emu: &mut OplEmuT) {
    emu.m_env_counter = 0;
    emu.m_status = 0;
    emu.m_timer_running = [0, 0];
    emu.m_active_channels = OPL_EMU_REGISTERS_ALL_CHANNELS;
    emu.m_modified_channels = OPL_EMU_REGISTERS_ALL_CHANNELS;
    emu.m_prepare_count = 0;

    opl_emu_registers_init(&mut emu.m_regs);

    for chnum in 0..OPL_EMU_REGISTERS_CHANNELS {
        opl_emu_fm_channel_init(
            &mut emu.m_channel[chnum],
            &emu.m_regs,
            opl_emu_registers_channel_offset(chnum as u32),
        );
    }

    for opnum in 0..OPL_EMU_REGISTERS_OPERATORS {
        emu.m_operator[opnum] = None;
    }

    opl_emu_assign_operators(emu);
}


