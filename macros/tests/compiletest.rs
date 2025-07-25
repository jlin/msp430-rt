#[test]
fn ui() {
    std::env::set_var("CARGO_BUILD_TARGET", "msp430-none-elf");
    std::env::set_var("CARGO_UNSTABLE_BUILD_STD", "std");
    
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
