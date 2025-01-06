

use std::convert::TryInto;
use std::usize;

const OPL_EMU_REGISTERS_CHANNELS: u32 = (1 << 3) - 1;
const OPL_EMU_REGISTERS_ALL_CHANNELS: u32 = (1 << (OPL_EMU_REGISTERS_CHANNELS + 1)) - 1;

