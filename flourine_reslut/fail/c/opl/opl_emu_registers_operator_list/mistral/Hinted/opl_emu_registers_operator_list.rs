
use std::u32;

fn opl_emu_registers_operator_list(o1: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    u32::from(o1) | (u32::from(o2) << 8) | (u32::from(o3) << 16) | (u32::from(o4) << 24)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_0() {
        assert_eq!(168956426, opl_emu_registers_operator_list(10, 18, 18, 10));
    }

    #[test]
    fn example_1() {
        assert_eq!(218761741, opl_emu_registers_operator_list(13, 10, 10, 13));
    }

    #[test]
    fn example_2() {
        assert_eq!(251691011, opl_emu_registers_operator_list(3, 128, 0, 15));
    }

    #[test]
    fn example_3() {
        assert_eq!(41549824, opl_emu_registers_operator_list(0, 0, 122, 2));
    }
}
