extern crate riscv_emu;

mod common;

use common::TestRun;

#[test]
fn test_parameters() {
    let mut t = TestRun::new("tests/programs/parameters.elf");
    t.write_byte(0xFA);
    t.write_string("foobar");
    t.write_halfword(0xBECA);
    t.write_word(0xCAFECAFE);
    let mut res = t.run();

    assert_eq!(res.read_byte(), 0xFA);
    assert_eq!(res.read_string(), "foobar");
    assert_eq!(res.read_halfword(), 0xBECA);
    assert_eq!(res.read_word(), 0xCAFECAFE);
}

#[test]
fn test_fibonacci() {
    fn fib_rust(n: u32) -> u32 {
        match n {
            1..=2 => 1,
            _ => fib_rust(n - 1) + fib_rust(n - 2),
        }
    }

    for i in &[1, 2, 10, 20, 33] {
        let mut t = TestRun::new("tests/programs/fibonacci.elf");
        t.write_byte(*i);
        let mut res = t.run();

        assert_eq!(res.read_word(), fib_rust(*i as u32));
    }
}
