use anyhow::{Context, Result};
use clap::{Arg, Command};
use rv32sim::simulator::Simulator;
use std::{fs::File, io::Read};

fn main() -> Result<()> {
    let command = Command::new("rv32sim: RISC-V(RV32I subset) Simulator")
        .author("wat-aro")
        .version("0.1.0")
        .arg(
            Arg::new("FILE")
                .required(true)
                .help("Target movable type file location."),
        )
        .get_matches();

    let filename = command
        .value_of("FILE")
        .context("No such file or directory")?;
    let mut data = Vec::new();
    File::open(filename)?.read_to_end(&mut data)?;

    let mut simulator = Simulator::new();
    simulator.initialize_memory(data);
    if let Err(e) = simulator.start() {
        eprintln!("Failed execute simulator: {}", e);
    }
    simulator.dump_registers();
    Ok(())
}
