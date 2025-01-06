

#[derive(Debug)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

#[derive(Debug)]
enum Op2Flags {
    FixedPitch = 1,
    Unused = 2,
    DoubleVoice = 4,
}

#[derive(Debug)]
struct OplEmuRegisters {
    lfo_am_counter: u16,
    lfo_pm_counter: u16,
    noise_lfsr: u32,
    lfo_am: u8,
    regdata: Vec<u8>,
    waveform: Vec<Vec<u16>>,
}

#[derive(Debug)]
struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: Vec<u8>,
    eg_shift: u8,
}

#[derive(Debug)]
struct OplEmuFmOperator {
    choffs: u32,
    opoffs: u32,
    phase: u32,
    env_attenuation: u16,
    env_state: OplEmuEnvelopeState,
    key_state: u8,
    keyon_live: u8,
    cache: OplEmuOpdataCache,
}

#[derive(Debug)]
struct OplEmuFmChannel {
    choffs: u32,
    feedback: Vec<i16>,
    feedback_in: i16,
}

#[derive(Debug)]
struct OplTimbre {
    modulator_e862: u32,
    carrier_e862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
}

#[derive(Debug)]
struct OplEmu {
    env_counter: u32,
    status: u8,
    timer_running: Vec<u8>,
    active_channels: u32,
    modified_channels: u32,
    prepare_count: u32,
    regs: OplEmuRegisters,
    channel: Vec<OplEmuFmChannel>,
    operator: Vec<OplEmuFmOperator>,
}

#[derive(Debug)]
struct Voicealloc {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8,
}

#[derive(Debug)]
struct Opl {
    notes2voices: Vec<Vec<Vec<Voicealloc>>>,
    channelpitch: Vec<u16>,
    channelvol: Vec<u16>,
    voices2notes: Vec<Voicealloc>,
    channelprog: Vec<u8>,
    opl3: i32,
    opl_emu: OplEmu,
    opl_gmtimbres: Vec<OplTimbre>,
    opl_gmtimbres_voice2: Vec<OplTimbre>,
    is_op2: i32,
    op2_flags: Vec<Op2Flags>,
}

impl Opl {
    fn opl_loadbank_internal(&mut self, file: &str, offset: i32) -> i32 {
        self.is_op2 = 0;
        let mut buff = vec![0u8; 16];
        let mut i: i32;
        let mut f = std::fs::File::open(file).unwrap();
        std::io::Read::read(&mut f, &mut buff).unwrap();
        if buff[0] != b'I' || buff[1] != b'B' || buff[2] != b'K' || buff[3] != 0x1A {
            return -3;
        }
        for i in offset..128 + offset {
            std::io::Read::read(&mut f, &mut buff).unwrap();
            self.opl_gmtimbres[i as usize].modulator_e862 = (buff[8] as u32) << 8;
            self.opl_gmtimbres[i as usize].modulator_e862 |= (buff[6] as u32) << 8;
            self.opl_gmtimbres[i as usize].modulator_e862 |= (buff[4] as u32) << 8;
            self.opl_gmtimbres[i as usize].modulator_e862 |= (buff[0] as u32);
            self.opl_gmtimbres[i as usize].carrier_e862 = (buff[9] as u32) << 8;
            self.opl_gmtimbres[i as usize].carrier_e862 |= (buff[7] as u32) << 8;
            self.opl_gmtimbres[i as usize].carrier_e862 |= (buff[5] as u32) << 8;
            self.opl_gmtimbres[i as usize].carrier_e862 |= (buff[1] as u32);
            self.opl_gmtimbres[i as usize].modulator_40 = buff[2];
            self.opl_gmtimbres[i as usize].carrier_40 = buff[3];
            self.opl_gmtimbres[i as usize].feedconn = buff[10];
            self.opl_gmtimbres[i as usize].finetune = buff[12] as i8;
            self.opl_gmtimbres[i as usize].notenum = 60;
            self.opl_gmtimbres[i as usize].noteoffset = 0;
        }
        0
    }

    fn opl_loadbank_ibk(&mut self, file: &str) -> i32 {
        let mut instruments = String::from(file);
        let mut percussion: Option<String> = None;
        let mut i: usize;
        for i in 0..instruments.len() {
            if instruments.chars().nth(i).unwrap() == ',' {
                instruments.truncate(i);
                percussion = Some(instruments[i + 1..].to_string());
                break;
            }
        }
        let res = self.opl_loadbank_internal(&instruments, 0);
        if res == 0 && percussion.is_some() {
            self.opl_loadbank_internal(&percussion.unwrap(), 128);
        }
        res
    }
}

