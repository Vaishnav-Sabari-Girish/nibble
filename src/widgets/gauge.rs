use clap::Args;

#[derive(Args, Debug)]
pub struct GaugeArgs {
    // TODO: Add gauge-specific flags
}

pub fn run(_args: GaugeArgs) -> anyhow::Result<()> {
    println!("Gauge widget coming soon!");
    Ok(())
}
