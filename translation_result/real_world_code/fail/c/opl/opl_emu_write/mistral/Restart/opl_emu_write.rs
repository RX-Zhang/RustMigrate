

use std::ops::Shr;
use std::usize;

type OplEmuKeyonType = u8;

fn opl_emu_fm_channel_keyonoff(fmch: &mut Box<dyn OplEmuFmChannelOpsTrait>, states: u32, ty: OplEmuKeyonType, chnum: u32) {
for (opnum, op) in fmch.m_op().iter_mut().enumerate() {
if (states & (1 << opnum)) != 0 {
let on = 1 << ty;
opl_emu_fm_operator_keyonoff(op, on, ty as u32);
}
}
}

trait OplEmuFmChannelOpsTrait {
fn m_op(&mut self) -> &mut Vec<Box<dyn OplEmuOperatorOpsTrait>>;
}

trait OplEmuOperatorOpsTrait {
fn keyon(&mut self, on: u32, ty: u32);
}

fn opl_emu_fm_operator_keyonoff(op: &mut Box<dyn OplEmuOperatorOpsTrait>, on: u32, ty: u32) {
op.keyon(on, ty);
}

