use std::{env, error::Error, fs, process::Command};
use svd2rust::{Config, Target};

fn main() -> Result<(), Box<dyn Error>> {
    const LINKER_SCRIPT_NAME: &str = "memory.x";
    const SVD_NAME: &str = "soc.svd";
    let out_dir = env::var("OUT_DIR")?;

    Command::new("litex_sim")
        .arg(format!("--output-dir={out_dir}/litex_sim"))
        .arg(format!("--csr-svd={out_dir}/{SVD_NAME}"))
        .arg(format!("--memory-x={out_dir}/{LINKER_SCRIPT_NAME}"))
        .arg("--cpu-variant=minimal")
        .arg("--no-compile")
        .status()?;

    println!("cargo:rustc-link-search={out_dir}"); // Let linker find the generated linker script

    let mut config = Config::default();
    config.target = Target::RISCV;
    config.make_mod = true;
    // Generate the svd file to a string in RAM
    let mut generation = svd2rust::generate(
        fs::read_to_string(format!("{out_dir}/{SVD_NAME}"))?.as_str(),
        &config
    )?;

    // Add newlines as the svd2rust utility do
    generation.lib_rs = generation.lib_rs.replace("] ", "]\n");

    // Write the generate peripheral access code
    fs::write("src/soc.rs", generation.lib_rs)?;

    Ok(())
}
