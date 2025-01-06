
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::mem;
use std::boxed::Box;

const OPL_EMU_REGISTERS_OPERATORS: usize = 36;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[repr(C)]
struct OplTimbre {
    modulator_E862: u32,
    carrier_E862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: u8,
    notenum: u8,
    noteoffset: i8,
}

struct Opl {
    is_op2: bool,
    opl_gmtimbres: Box<[OplTimbre; 256]>,
}

fn opl_loadbank_internal(
    opl: &mut Opl,
    file: &str,
    offset: usize,
) -> io::Result<()> {
    opl.is_op2 = false;
    let mut buff = [0u8; 16];
    let mut file = File::open(file)?;
    file.seek(SeekFrom::End(0))?;
    if file.seek(SeekFrom::Current(0))? != 3204 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid file size",
        ));
    }
    file.seek(SeekFrom::Start(0))?;
    file.read_exact(&mut buff)?;
    if &buff[..4] != b"IBK\x1A" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid file header",
        ));
    }
    for i in offset..128 + offset {
        file.read_exact(&mut buff)?;
        opl.opl_gmtimbres[i].modulator_E862 =
            (buff[8] as u32) << 24 | (buff[6] as u32) << 16 | (buff[4] as u32) << 8 | buff[0] as u32;
        opl.opl_gmtimbres[i].carrier_E862 =
            (buff[9] as u32) << 24 | (buff[7] as u32) << 16 | (buff[5] as u32) << 8 | buff[1] as u32;
        opl.opl_gmtimbres[i].modulator_40 = buff[2];
        opl.opl_gmtimbres[i].carrier_40 = buff[3];
        opl.opl_gmtimbres[i].feedconn = buff[10];
        opl.opl_gmtimbres[i].finetune = buff[12];
        opl.opl_gmtimbres[i].notenum = 60;
        opl.opl_gmtimbres[i].noteoffset = 0;
    }
    Ok(())
}

fn opl_loadbank_ibk(opl: &mut Opl, file: &str) -> io::Result<()> {
    let mut instruments = String::new();
    let mut percussion = String::new();
    let mut res: Result<(), io::Error> = Ok(());
    File::open(file)?.read_to_string(&mut instruments)?;
    for (i, c) in instruments.chars().enumerate() {
        if c == ',' {
            instruments.truncate(i);
            percussion = instruments[i + 1..].to_string();
            break;
        }
    }
    res = opl_loadbank_internal(opl, &instruments, 0);
    if res.is_ok() && !percussion.is_empty() {
        opl_loadbank_internal(opl, &percussion, 128)?;
    }
    res
}

