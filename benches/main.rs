use rand::Rng;

fn bench_main(c: &mut criterion::Criterion) {
    let mut rng = rand::thread_rng();
    let mut buf = [0; 4096];
    rng.fill(&mut buf);

    c.bench_function("bench_blake2b_rs", |b| {
        b.iter(|| {
            let mut h = blake2b_rs::Blake2bBuilder::new(32).personal(b"ckb-default-hash").build();
            h.update(&buf);
            let mut r = [0; 32];
            h.finalize(&mut r);
            criterion::black_box(r);
        })
    });

    c.bench_function("bench_blake2ya", |b| {
        b.iter(|| {
            let mut p = blake2ya::blake2b_params();
            p.digest(32);
            p.person(b"ckb-default-hash");
            let mut h = blake2ya::blake2b(p);
            h.update(&buf);
            let mut r = [0; 32];
            h.digest(&mut r);
            criterion::black_box(r);
        })
    });
}

criterion::criterion_group!(benches, bench_main);
criterion::criterion_main!(benches);
