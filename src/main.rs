mod spac_api;
mod spac_cli;

fn main () -> Result<(), Box<dyn std::error::Error>>
{ spac_cli::spac_cli() }
