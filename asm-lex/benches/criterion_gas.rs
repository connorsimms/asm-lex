use asm_lex::source::lexer::Lexer;
use asm_lex::source::{
    gas::{targets::*, Gas},
    Dialect, Item,
};
use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode, Throughput,
};
use std::hint::black_box;

fn path(file: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("benches/fixtures")
        .join(file)
}

fn bench_target<D: Dialect>(c: &mut Criterion, group_name: &str, dir: &str) {
    let mut group = c.benchmark_group(group_name);

    group.sampling_mode(SamplingMode::Flat);

    for file in ["small.s", "medium.s", "large.s"] {
        let bytes = std::fs::read(path(&format!("{dir}/{file}"))).unwrap();
        group.throughput(Throughput::Bytes(bytes.len() as u64));
        group.bench_with_input(BenchmarkId::new("lex_drain", file), &bytes, |b, bytes| {
            b.iter(|| {
                let mut n = 0usize;
                for item in Lexer::<D>::new(black_box(bytes)) {
                    n += black_box(&item).span().len();
                }
                n
            });
        });
        group.bench_with_input(BenchmarkId::new("lex_collect", file), &bytes, |b, bytes| {
            b.iter(|| Lexer::<D>::new(black_box(bytes)).collect::<Vec<Item>>());
        });
    }
}

fn all(c: &mut Criterion) {
    bench_target::<Gas<X86_64LinuxElf>>(c, "x86_64_linux_elf", "x86_64_linux_elf");
    // bench_target::<Gas<Aarch64LinuxElf>>(c, "aarch64_linux_elf", "aarch64_linux_elf");
    // bench_target::<Gas<ArmLinuxEabiElf>>(c, "arm_linux_eabi_elf", "arm_linux_eabi_elf");
    // bench_target::<Gas<Riscv64LinuxElf>>(c, "riscv64_linux_elf", "riscv64_linux_elf");
}

criterion_group!(benches, all);
criterion_main!(benches);
