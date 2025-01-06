

use libc::uint32_t;

fn opl_emu_registers_operator_offset(opnum: uint32_t) -> uint32_t {
    let opnum_mod_18 = (opnum % 18) as uint32_t;
    let opnum_div_18 = (opnum / 18) as uint32_t;
    let result = (opnum_mod_18.wrapping_add(2 * (opnum_mod_18 / 6)).wrapping_add(0x100 * opnum_div_18)) as uint32_t;
    result
}

fn main() {
    let test_cases = vec![248565999u32, 68943876u32, 1111523033u32, 3808456491u32];
    for test_case in test_cases {
        println!("{}", opl_emu_registers_operator_offset(test_case));
    }
}

