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

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn normalize(s: &str) -> String {
    let mut out: String = s
        .lines()
        .map(|line| {
            if line.trim_start().starts_with(".ident") {
                let indent = &line[..line.len() - line.trim_start().len()];
                format!("{indent}.ident\t\"<toolchain>\"")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    out.push('\n');
    out
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
        return Err(format!(
            "{} {} {}: {}",
            t.cc,
            t.dir,
            v.name,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(normalize(&String::from_utf8_lossy(&output.stdout)))
}

fn main() {
    let check = std::env::args().any(|a| a == "--check");
    let filter: Option<String> = std::env::args().skip(1).find(|a| !a.starts_with("--"));

    for (var, value) in std::env::vars() {
        println!("{var}: {value}");
    }

    let mut failures = Vec::new();
    let mut stale = Vec::new();

    for t in TARGETS {
        if let Some(f) = &filter {
            if !t.dir.contains(f.as_str()) && t.dialect != f.as_str() {
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
                Err(e) => failures.push(e),
                Ok(content) => {
                    let existing = std::fs::read_to_string(&path).ok();
                    if existing.as_deref() == Some(content.as_str()) {
                        continue;
                    }
                    if check {
                        stale.push(path.display().to_string());
                    } else {
                        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                        std::fs::write(&path, &content).unwrap();
                        println!("wrote {}", path.display());
                    }
                }
            }
        }
    }

    for f in &failures {
        eprintln!("error: {f}");
    }
    for s in &stale {
        eprintln!("out of date: {s}");
    }
    if !failures.is_empty() || !stale.is_empty() {
        std::process::exit(1);
    }
}
