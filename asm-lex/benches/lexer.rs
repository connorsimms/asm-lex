use std::hint::black_box;
use std::path::PathBuf;

use asm_lex::source::gas::{targets::X86LinuxElf, Gas};
use asm_lex::source::lexer::Lexer;
use asm_lex::source::llvm::{targets::X86Elf, Llvm};
use asm_lex::source::{Dialect, Item};
use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode, Throughput,
};

const VARIANTS: &[&str] = &["baseline", "dense", "verbose", "debug"];

const SIZES: &[&str] = &["10KiB", "100KiB", "1MiB"];

const ITEMS_SIZE: &str = "1MiB";

fn bench_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate should have a parent directory")
        .join("target/bench-fixtures")
}

fn load(dialect: &str, target: &str, variant: &str, size: &str) -> Vec<u8> {
    let path = bench_root()
        .join(dialect)
        .join(target)
        .join(format!("{variant}.{size}.s"));
    std::fs::read(&path).unwrap_or_else(|e| {
        panic!(
            "missing benchmark input {}: {e}\nrun `cargo xtask bench` first",
            path.display()
        )
    })
}

fn bench_bytes<D: Dialect>(c: &mut Criterion, dialect: &str, target: &str) {
    let mut group = c.benchmark_group(format!("{dialect}/{target}/bytes"));
    group.sampling_mode(SamplingMode::Flat);

    for variant in VARIANTS {
        for size in SIZES {
            let data = load(dialect, target, variant, size);
            group.throughput(Throughput::Bytes(data.len() as u64));
            group.bench_with_input(BenchmarkId::new(*variant, size), &data, |b, data| {
                b.iter(|| {
                    let mut n = 0usize;
                    for item in Lexer::<D>::new(black_box(data)) {
                        n += black_box(&item).span().len();
                    }
                    n
                });
            });
        }
    }
    group.finish();
}

fn bench_items<D: Dialect>(c: &mut Criterion, dialect: &str, target: &str) {
    let mut group = c.benchmark_group(format!("{dialect}/{target}/items"));
    group.sampling_mode(SamplingMode::Flat);

    for variant in VARIANTS {
        let data = load(dialect, target, variant, ITEMS_SIZE);
        let items = Lexer::<D>::new(&data).count();
        group.throughput(Throughput::Elements(items as u64));
        group.bench_with_input(BenchmarkId::new(*variant, ITEMS_SIZE), &data, |b, data| {
            b.iter(|| {
                let mut n = 0usize;
                for item in Lexer::<D>::new(black_box(data)) {
                    n += black_box(&item).span().len();
                }
                n
            });
        });
    }
    group.finish();
}

fn bench_collect<D: Dialect>(c: &mut Criterion, dialect: &str, target: &str) {
    let mut group = c.benchmark_group(format!("{dialect}/{target}/collect"));
    group.sampling_mode(SamplingMode::Flat);

    let data = load(dialect, target, "dense", ITEMS_SIZE);
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_with_input(BenchmarkId::new("dense", ITEMS_SIZE), &data, |b, data| {
        b.iter(|| Lexer::<D>::new(black_box(data)).collect::<Vec<Item>>());
    });
    group.finish();
}

fn all(c: &mut Criterion) {
    bench_bytes::<Gas<X86LinuxElf>>(c, "gas", "x86_linux_elf");
    bench_items::<Gas<X86LinuxElf>>(c, "gas", "x86_linux_elf");
    bench_collect::<Gas<X86LinuxElf>>(c, "gas", "x86_linux_elf");

    bench_bytes::<Llvm<X86Elf>>(c, "llvm", "x86_elf");
    bench_items::<Llvm<X86Elf>>(c, "llvm", "x86_elf");
    bench_collect::<Llvm<X86Elf>>(c, "llvm", "x86_elf");
}

criterion_group!(benches, all);
criterion_main!(benches);
