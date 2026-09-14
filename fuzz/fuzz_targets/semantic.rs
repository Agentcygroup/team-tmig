#![no_main]
use libfuzzer_sys::fuzz_target;
fuzz_target!(|data: &[u8]| {
    if data.len() >= 8 {
        let _ = tmig::boundary::semantic::classify(0.0, 0.0, 0.0);
    }
});
