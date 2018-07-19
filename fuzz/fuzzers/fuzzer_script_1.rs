#![no_main]
extern crate libfuzzer_sys;
extern crate dtparse;
use dtparse::parse;
#[export_name="rust_fuzzer_test_input"]
pub extern fn go(data: &[u8]) {
    if let Ok(s) = std::str::from_utf8(data) {
        parse(s);
    }
}
