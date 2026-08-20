#![allow(clippy::must_use_candidate)]

mod bench;

use std::path::PathBuf;
use std::process::Command;

struct Variant {
    name: &'static str,
    source: &'static str,
    flags: &'static [&'static str],
}

const VARIANTS: &[Variant] = &[
    Variant {
        name: "baseline",
        source: "hello-world.c",
        flags: &["-S", "-O0", "-fno-verbose-asm"],
    },
    Variant {
        name: "verbose",
        source: "hello-world.c",
        flags: &["-S", "-O0", "-fverbose-asm"],
    },
    Variant {
        name: "inline-asm",
        source: "hello-world-inline-asm.c",
        flags: &["-S", "-O0", "-fno-verbose-asm"],
    },
];

struct Target {
    dialect: &'static str,
    dir: &'static str,
    cc: &'static str,
    args: &'static [&'static str],
}

const TARGETS: &[Target] = &[
    Target {
        dialect: "llvm",
        dir: "x86_elf",
        cc: "clang",
        args: &["--target=x86_64-unknown-linux-gnu"],
    },
    Target {
        dialect: "llvm",
        dir: "x86_darwin",
        cc: "clang",
        args: &["--target=x86_64-apple-macosx"],
    },
    Target {
        dialect: "llvm",
        dir: "aarch64_elf",
        cc: "clang",
        args: &["--target=aarch64-unknown-linux-gnu"],
    },
    Target {
        dialect: "llvm",
        dir: "arm_elf",
        cc: "clang",
        args: &["--target=armv7-unknown-linux-gnueabihf"],
    },
    Target {
        dialect: "llvm",
        dir: "riscv_elf",
        cc: "clang",
        args: &["--target=riscv64-unknown-linux-gnu"],
    },
    Target {
        dialect: "llvm",
        dir: "aarch64_darwin",
        cc: "clang",
        args: &["--target=arm64-apple-macosx"],
    },
    Target {
        dialect: "gas",
        dir: "x86_linux_elf",
        cc: "x86_64-unknown-linux-gnu-gcc",
        args: &["-frandom-seed=0"],
    },
    Target {
        dialect: "gas",
        dir: "aarch64_linux_elf",
        cc: "aarch64-unknown-linux-gnu-gcc",
        args: &["-frandom-seed=0"],
    },
    Target {
        dialect: "gas",
        dir: "arm_linux_eabi_elf",
        cc: "armv7l-unknown-linux-gnueabihf-gcc",
        args: &["-frandom-seed=0"],
    },
    Target {
        dialect: "gas",
        dir: "riscv_generic_elf",
        cc: "riscv64-unknown-linux-gnu-gcc",
        args: &["-frandom-seed=0"],
    },
];

const BASE: &[&str] = &["-ffreestanding"];

/// # Panics
pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn generate(t: &Target, v: &Variant) -> Result<String, String> {
    let src_dir = root().join("asm-lex/tests/fixtures");
    let output = Command::new(t.cc)
        .env_clear()
        .env("PATH", std::env::var("PATH").unwrap())
        .env("LC_ALL", "C")
        .current_dir(&src_dir)
        .args(BASE)
        .args(t.args)
        .args(v.flags)
        .arg("-o")
        .arg("-")
        .arg(v.source)
        .output()
        .map_err(|e| format!("failed to run {}: {e}", t.cc))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "{} {} {}: {}",
            t.cc,
            t.dir,
            v.name,
            stderr.trim().lines().next_back().unwrap_or("(no stderr)")
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn fixtures(check: bool, filter: Option<&str>) -> Vec<String> {
    let mut errors = Vec::new();

    for t in TARGETS {
        if let Some(f) = filter {
            if !t.dir.contains(f) && t.dialect != f {
                continue;
            }
        }
        for v in VARIANTS {
            let path = root()
                .join("asm-lex/tests/fixtures")
                .join(t.dialect)
                .join(t.dir)
                .join(format!("{}.s", v.name));

            match generate(t, v) {
                Err(e) => errors.push(e),
                Ok(content) => {
                    if std::fs::read_to_string(&path).ok().as_deref() == Some(content.as_str()) {
                        continue;
                    }
                    if check {
                        errors.push(format!("out of date: {}", path.display()));
                    } else {
                        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                        std::fs::write(&path, &content).unwrap();
                        println!("wrote {}", path.display());
                    }
                }
            }
        }
    }
    errors
}

const HELP: &str = "\
cargo xtask <command> [--check] [--force] [filter]

  fixtures   regenerate committed snapshot fixtures
  bench      download, compile, and slice benchmark inputs
";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let check = args.iter().any(|a| a == "--check");
    let force = args.iter().any(|a| a == "--force");
    let filter: Option<&str> = args
        .iter()
        .skip(1)
        .find(|a| !a.starts_with("--"))
        .map(String::as_str);

    let errors = match args.first().map_or("help", String::as_str) {
        "fixtures" => fixtures(check, filter),
        "bench" => bench::run(force, filter),
        _ => {
            print!("{HELP}");
            return;
        }
    };

    for e in &errors {
        eprintln!("error: {e}");
    }
    if !errors.is_empty() {
        std::process::exit(1);
    }
}
