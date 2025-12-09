use clap::Args;

#[derive(Args, Debug)]
pub struct TableArgs {
    // TODO: Add table-specific flags
}

pub fn run(_args: TableArgs) -> anyhow::Result<()> {
    println!("Table widget coming soon!");
    Ok(())
}
