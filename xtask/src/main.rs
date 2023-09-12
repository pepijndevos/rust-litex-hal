use anyhow::Result;
use clap::{Args, Parser};
use xshell::{cmd, Shell};

#[derive(Parser)]
enum Cli {
    Simulate(Simulate),
}

#[derive(Args)]
struct Simulate {
    /// Example to simulate.
    #[arg(long, short)]
    example: String,

    /// Build artifacts in release mode, with optimizations.
    #[arg(long, short)]
    release: bool,
}

fn main() -> Result<()> {
    const TARGET: &str = "riscv32i-unknown-none-elf";

    let Cli::Simulate(Simulate { example, release }) = Cli::parse();

    let release_flag = release.then_some("--release");

    let sh = Shell::new()?;
    cmd!(
        sh,
        "cargo build {release_flag...} --target {TARGET} --example {example}"
    )
    .run()?;

    let profile_dir = if release { "release" } else { "debug" };
    let example_path = format!("target/{TARGET}/{profile_dir}/examples/{example}");
    cmd!(
        sh,
        "riscv64-elf-objcopy {example_path} -O binary {example_path}.bin"
    )
    .run()?;

    cmd!(
        sh,
        "litex_sim --output-dir=target/litex_sim --cpu-variant=minimal --rom-init={example_path}.bin --non-interactive" 
    )
    .run()?;

    Ok(())
}
