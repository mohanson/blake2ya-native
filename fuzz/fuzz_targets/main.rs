#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: Vec<Vec<u8>>| {
    let result = {
        let mut p = blake2ya::blake2b_params();
        p.digest(32);
        p.person(b"ckb-default-hash");
        let mut h = blake2ya::blake2b(p);
        for elem in &data {
            h.update(elem);
        }
        let mut r = [0; 32];
        h.digest(&mut r);
        r
    };
    let expect = {
        let mut h = blake2b_rs::Blake2bBuilder::new(32).personal(b"ckb-default-hash").build();
        for elem in &data {
            h.update(elem);
        }
        let mut r = [0u8; 32];
        h.finalize(&mut r);
        r
    };
    assert_eq!(result, expect);
});
