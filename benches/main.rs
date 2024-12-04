fn bench_blake2b_rs(c: &mut criterion::Criterion) {
    let data = [0; 4096];
    c.bench_function("bench_blake2b_rs", |b| {
        b.iter(|| {
            let mut h = blake2b_rs::Blake2bBuilder::new(32).personal(b"ckb-default-hash").build();
            h.update(&data);
            let mut r = [0; 32];
            h.finalize(&mut r);
            criterion::black_box(r);
        })
    });
}

fn bench_blake2ya(c: &mut criterion::Criterion) {
    let data = [0; 4096];
    c.bench_function("bench_blake2ya", |b| {
        b.iter(|| {
            let mut p = blake2ya::blake2b_params();
            p.digest(32);
            p.person(b"ckb-default-hash");
            let mut h = blake2ya::blake2b(p);
            h.update(&data);
            let mut r = [0; 32];
            h.digest(&mut r);
            criterion::black_box(r);
        })
    });
}

criterion::criterion_group!(benches, bench_blake2b_rs, bench_blake2ya,);
criterion::criterion_main!(benches);
