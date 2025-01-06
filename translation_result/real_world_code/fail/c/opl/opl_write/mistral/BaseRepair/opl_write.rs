
use std::ops::BitAnd;

const OPL_U_REGISTERSS: usize = 9;
constPL_EMU_ISTERS_ALL_CHANNELS: u32 = (1 << OPL_EMU_REGISTERS_CHANNELS) - 1;
const OPL_EMU_REGISTERS_RHYTHM_CHANNEL: u32 = 0xff;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_REG_MODE: usize = 0x04;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Clone, Copy, PartialEq, Eq)]
enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
    OPL_EMU_EG_STATES = 6,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum OplEmuKeyonType {
    OPL_EMU_KEYON_NORMAL = 0,
    OPL_EMU_KEYON_RHYTHM = 1,
    OPL_EMU_KEYON_CSM = 2,
}

const OP2_FIXEDPITCH: u32 = 1;
const OP2_UNUSED: u32 = 2;
const OP2_DOUBLEVOICE: u32 = 4;

#[derive(Default)]
struct OplEmuFmOperator {
    m_keyon_live: u8,
}

#[derive(Default)]
struct OplEmuFmChannel {
    m_op: [Option<OplEmuFmOperator>; 4],
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
    m_modified_channels: u32,
    m_channel: [OplEmuFmChannel; OPL_EMU_REGISTERS_ALL_CHANNELS as usize],
}

impl OplEmuRegisters {
    fn write(&mut self, index: u16, data: u8) -> (u32, u32) {
        if index == OPL_EMU_REGISTERS_REG_MODE && (data & 0x80) != 0 {
            self.m_regdata[index as usize] |= 0x80;
        } else {
            self.m_regdata[index as usize] = data;
        }

        if index == 0xbd {
            return (OPL_EMU_REGISTERS_RHYTHM_CHANNEL, (data & 0x1f) as u32);
        }

        if (index & 0xf0) == 0xb0 {
            let channel = (index & 0x0f) as u32;
            if channel < 9 {
                channel += 9 * (data & 0x08) as u32;
                return (channel, if data & 0x20 != 0 { 15 } else { 0 });
            }
        }

        (0, 0)
    }
}

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_fm_operator_keyonoff(fmop: &mut OplEmuFmOperator, on: u32, ty: OplEmuKeyonType) {
    fmop.m_keyon_live = (fmop.m_keyon_live & !(1 << (ty as u32))) | (opl_emu_bitfield(on, 0, 1) << (ty as u32));
}

fn opl_emu_fm_channel_keyonoff(fmch: &mut OplEmuFmChannel, states: u32, ty: OplEmuKeyonType, chnum: u32) {
    for opnum in 0..4 {
        if fmch.m_op[opnum].is_some() {
            opl_emu_fm_operator_keyonoff(fmch.m_op[opnum].as_mut().unwrap(), opl_emu_bitfield(states, opnum, 1), ty);
        }
    }
}

fn opl_write(
    opl: &mut Box<OplEmuRegisters>,
    count: usize,
    regs: &[u16],
    data: &[u8],
) {
    let mut emu = opl.as_mut();
    for (i, regnum) in regs.iter().enumerate() {
        let value = data[i];

        if *regnum == OPL_EMU_REGISTERS_REG_MODE {
            continue;
        }

        emu.m_modified_channels = OPL_EMU_REGISTERS_ALL_CHANNELS;
        let (keyon_channel, keyon_opmask) = emu.write(*regnum, value);

        if keyon_channel < OPL_EMU_REGISTERS_CHANNELS as u32 {
            opl_emu_fm_channel_keyonoff(
                &mut emu.m_channel[keyon_channel as usize],
                keyon_opmask,
                OplEmuKeyonType::OPL_EMU_KEYON_NORMAL,
                keyon_channel,
            );
        } else if OPL_EMU_REGISTERS_CHANNELS as u32 >= 9 && keyon_channel == OPL_EMU_REGISTERS_RHYTHM_CHANNEL {
            opl_emu_fm_channel_keyonoff(
                &mut emu.m_channel[6],
                if keyon_opmask & 0x10 != 0 { 3 } else { 0 },
                OplEmuKeyonType::OPL_EMU_KEYON_RHYTHM,
                6,
            );
            opl_emu_fm_channel_keyonoff(
                &mut emu.m_channel[7],
                (keyon_opmask & 0x01) | ((keyon_opmask & 0x08) << 1),
                OplEmuKeyonType::OPL_EMU_KEYON_RHYTHM,
                7,
            );
            opl_emu_fm_channel_keyonoff(
                &mut emu.m_channel[8],
                (keyon_opmask & 0x04) | ((keyon_opmask & 0x02) << 1),
                OplEmuKeyonType::OPL_EMU_KEYON_RHYTHM,
                8,
            );
        }
    }
}

