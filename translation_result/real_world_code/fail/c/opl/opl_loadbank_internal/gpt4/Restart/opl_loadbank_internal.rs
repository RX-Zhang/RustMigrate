
use std::fs::File;
use std::io::{self, Read};

struct Opl {
    is_op2: i32,
    opl_gmtimbres: Box<[OplGmtimbre; 128]>,
}

struct OplGmtimbre {
    modulator_E862: u32,
    carrier_E862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i32,
}

fn opl_loadbank_internal(opl: &mut Opl, file: &str, offset: usize) -> i32 {
    opl.is_op2 = 0;
    let mut buff = [0u8; 16];
    let mut f = match File::open(file) {
        Ok(file) => file,
        Err(_) => return -1,
    };
    
    let file_size = match f.metadata() {
        Ok(metadata) => metadata.len() as usize,
        Err(_) => return -1,
    };

    if file_size != 3204 {
        return -2;
    }

    if f.read_exact(&mut buff[..4]).is_err() || buff[0] != b'I' || buff[1] != b'B' || buff[2] != b'K' || buff[3] != 0x1A {
        return -3;
    }

    for (i, item) in opl.opl_gmtimbres.iter_mut().enumerate().take(128).skip(offset) {
        if f.read_exact(&mut buff).is_err() {
            return -1;
        }
        
        item.modulator_E862 = (u32::from(buff[8]) << 24)
                               | (u32::from(buff[6]) << 16)
                               | (u32::from(buff[4]) << 8)
                               | u32::from(buff[0]);

        item.carrier_E862 = (u32::from(buff[9]) << 24)
                            | (u32::from(buff[7]) << 16)
                            | (u32::from(buff[5]) << 8)
                            | u32::from(buff[1]);

        item.modulator_40 = buff[2];
        item.carrier_40 = buff[3];
        item.feedconn = buff[10];
        item.finetune = buff[11] as i8;
        item.notenum = 60;
        item.noteoffset = 0;
    }

    0
}
