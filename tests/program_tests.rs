extern crate riscv_emu;

mod common;

use common::TestRun;

#[test]
fn test_parameters() {
    let mut res = TestRun::new("tests/programs/parameters.elf")
        .write_byte(0xFA)
        .write_string("foobar")
        .write_halfword(0xBECA)
        .write_word(0xCAFECAFE)
        .run();

    assert_eq!(res.read_byte(), 0xFA);
    assert_eq!(res.read_string(), "foobar");
    assert_eq!(res.read_halfword(), 0xBECA);
    assert_eq!(res.read_word(), 0xCAFECAFE);
}
