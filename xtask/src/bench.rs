use std::fmt::Write;
use std::path::PathBuf;
use std::process::Command;

use sha3::{Digest, Sha3_256};

use asm_lex::source::gas::{targets::x86::X86LinuxElf, Gas};
use asm_lex::source::llvm::{targets::x86::X86Elf, Llvm};
use asm_lex::{source::lexer::Lexer, source::Dialect};

const URL: &str = "https://sqlite.org/2026/sqlite-amalgamation-3530400.zip";
const SHA3_256: &str = "628a44cfe82c66aed1ccbbe85a562d2e33ebe64b3288981ed76285612227934e";
const SOURCE: &str = "sqlite3.c";

const SIZES: &[(&str, usize)] = &[
    ("10KiB", 10 * 1024),
    ("100KiB", 100 * 1024),
    ("1MiB", 1024 * 1024),
];

struct BenchVariant {
    name: &'static str,
    flags: &'static [&'static str],
}

const BENCH_VARIANTS: &[BenchVariant] = &[
    BenchVariant {
        name: "baseline",
        flags: &["-O0", "-fno-verbose-asm"],
    },
    BenchVariant {
        name: "dense",
        flags: &["-O2", "-fno-verbose-asm"],
    },
    BenchVariant {
        name: "verbose",
        flags: &["-O2", "-fverbose-asm"],
    },
    BenchVariant {
        name: "debug",
        flags: &["-O0", "-g", "-fno-verbose-asm"],
    },
];

fn dir() -> PathBuf {
    crate::root().join("target/bench-fixtures")
}

struct BenchTarget {
    dialect: &'static str,
    dir: &'static str,
    cc: &'static str,
    gas: bool,
}

const BENCH_TARGETS: &[BenchTarget] = &[
    BenchTarget {
        dialect: "gas",
        dir: "x86_linux_elf",
        cc: "gcc",
        gas: true,
    },
    BenchTarget {
        dialect: "llvm",
        dir: "x86_elf",
        cc: "clang",
        gas: false,
    },
];

fn sh(program: &str, args: &[&std::ffi::OsStr]) -> Result<(), String> {
    let out = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("{program}: {e}"))?;
    if out.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&out.stderr);
    Err(format!(
        "{program}: {}",
        stderr.trim().lines().next_back().unwrap_or("failed")
    ))
}

fn ensure_source() -> Result<PathBuf, String> {
    let src = dir().join(SOURCE);
    if src.is_file() {
        return Ok(src);
    }
    std::fs::create_dir_all(dir()).map_err(|e| e.to_string())?;

    let zip = dir().join("amalgamation.zip");
    println!("downloading {URL}");
    sh(
        "curl",
        &[
            "-sSL".as_ref(),
            "--fail".as_ref(),
            "-o".as_ref(),
            zip.as_ref(),
            URL.as_ref(),
        ],
    )?;

    let bytes = std::fs::read(&zip).map_err(|e| e.to_string())?;
    let actual = format!("{:x}", Sha3_256::digest(&bytes));
    if actual != SHA3_256 {
        let _ = std::fs::remove_file(&zip);
        return Err(format!(
            "sha3-256 mismatch\n  expected {SHA3_256}\n  actual   {actual}"
        ));
    }

    sh(
        "unzip",
        &[
            "-jqo".as_ref(),
            zip.as_ref(),
            "*sqlite3.c".as_ref(),
            "-d".as_ref(),
            dir().as_ref(),
        ],
    )?;
    let _ = std::fs::remove_file(&zip);
    println!("cached {}", src.display());
    Ok(src)
}

fn compile(target: &BenchTarget, variant: &BenchVariant, force: bool) -> Result<PathBuf, String> {
    let out_dir = dir().join(target.dialect).join(target.dir);
    let out = out_dir.join(format!("{}.s", variant.name));

    if out.is_file() && !force {
        return Ok(out);
    }

    std::fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;

    println!(
        "compiling for {}/{}/{}.s",
        target.dialect, target.dir, variant.name
    );

    // Use environment variables, system headers needed
    let status = Command::new(target.cc)
        .current_dir(dir())
        .arg("-S")
        .args(variant.flags)
        .arg("-o")
        .arg(&out)
        .arg(SOURCE)
        .status()
        .map_err(|e| format!("{}: {e}", target.cc))?;
    if !status.success() {
        return Err(format!(
            "{} {}/{}/{}: compile failed",
            target.cc, target.dialect, target.dir, variant.name
        ));
    }
    Ok(out)
}

fn truncate<D: Dialect>(data: &[u8], limit: usize) -> (usize, usize) {
    let mut end = 0;
    let mut items = 0;
    for item in Lexer::<D>::new(data) {
        if item.span().end > limit {
            break;
        }
        end = item.span().end;
        items += 1;
    }
    if data.get(end) == Some(&b'\n') {
        end += 1;
    }
    (end, items)
}

fn count_items<D: Dialect>(data: &[u8]) -> usize {
    Lexer::<D>::new(data).count()
}

pub fn run(force: bool, filter: Option<&str>) -> Vec<String> {
    let mut errors = Vec::new();

    if let Err(e) = ensure_source() {
        return vec![e];
    }

    let mut manifest = String::from("# name  bytes  items\n");

    for target in BENCH_TARGETS {
        for variant in BENCH_VARIANTS {
            if let Some(f) = filter {
                if !target.dialect.contains(f)
                    && !target.dir.contains(f)
                    && !variant.name.contains(f)
                {
                    continue;
                }
            }

            let path = match compile(target, variant, force) {
                Ok(p) => p,
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            };

            let data = match std::fs::read(&path) {
                Ok(d) => d,
                Err(e) => {
                    errors.push(format!("read {}: {e}", path.display()));
                    continue;
                }
            };

            let items = if target.gas {
                count_items::<Gas<X86LinuxElf>>(&data)
            } else {
                count_items::<Llvm<X86Elf>>(&data)
            };

            let _ = writeln!(
                manifest,
                "{}/{}/{}.s {} {}",
                target.dialect,
                target.dir,
                variant.name,
                data.len(),
                items
            );

            for (label, limit) in SIZES {
                let (end, items) = if target.gas {
                    truncate::<Gas<X86LinuxElf>>(&data, *limit)
                } else {
                    truncate::<Llvm<X86Elf>>(&data, *limit)
                };
                if end == 0 {
                    errors.push(format!(
                        "{}/{}/{}: nothing fits in {label}",
                        target.dialect, target.dir, variant.name
                    ));
                    continue;
                }
                let out = dir()
                    .join(target.dialect)
                    .join(target.dir)
                    .join(format!("{}.{label}.s", variant.name));
                if let Err(e) = std::fs::write(&out, &data[..end]) {
                    errors.push(format!("write {}: {e}", out.display()));
                    continue;
                }
                let _ = writeln!(
                    manifest,
                    "{}/{}/{}.{label}.s {end} {items}",
                    target.dialect, target.dir, variant.name
                );
            }
        }
    }

    let manifest_path = dir().join("MANIFEST.txt");
    if let Err(e) = std::fs::write(&manifest_path, &manifest) {
        errors.push(format!("write manifest: {e}"));
    }
    println!("\n{manifest}");
    errors
}
