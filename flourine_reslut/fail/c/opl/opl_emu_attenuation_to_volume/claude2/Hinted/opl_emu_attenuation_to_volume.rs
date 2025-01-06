
use std::boxed::Box;
use once_cell::sync::Lazy;

static S_POWER_TABLE: Lazy<Box<[u16; 256]>> = Lazy::new(|| {
    let mut table = Box::new([0u16; 256]);
    for i in 0..256 {
        table[i] = (((i as u32 | 0x400) << 2) & 0xffff) as u16;
    }
    table
});

fn opl_emu_attenuation_to_volume(input: u32) -> u32 {
    let table = &*S_POWER_TABLE;
    let fractional = input as usize & 0xff;
    let whole = (input as usize).wrapping_sub(fractional) >> 8;
    
    (table[fractional].wrapping_sub(whole as u16) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(opl_emu_attenuation_to_volume(34210341), 1848);
    }

    #[test]
    fn test_1() {
        assert_eq!(opl_emu_attenuation_to_volume(168453385), 249);
    }
}

