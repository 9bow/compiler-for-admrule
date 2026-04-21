//! `admrule-kr-compiler` — scaffold-only bootstrap.
//!
//! The actual `.cache/admrule/*.xml` → bare Git repo pipeline is not yet implemented.
//! This binary exists so the repository skeleton compiles cleanly and can be published
//! to `9bow/compiler-for-admrule` as a placeholder for follow-up work, mirroring the
//! `compiler-for-precedent` structure.
//!
//! Follow-up: see [`RESOURCE_ADMRULE.md`](../RESOURCE_ADMRULE.md) §"파이프라인 패키지
//! 설계" and `.omc/plans/admrule-kr-bootstrap.md` for the intended architecture.

use std::path::PathBuf;

use anyhow::{Result, bail};
use clap::Parser;

/// Compile cached admrule XML into a bare Git repository.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// Path to the input `.cache/admrule/` directory (contains one `{행정규칙일련번호}.xml` per record).
    input: PathBuf,

    /// Output bare repository path.
    #[arg(short, long, default_value = "./output.git")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    bail!(
        "admrule-kr-compiler is not yet implemented (scaffold only).\n\
         Input: {}\n\
         Output: {}\n\
         Follow RESOURCE_ADMRULE.md for the intended design.",
        cli.input.display(),
        cli.output.display(),
    );
}
