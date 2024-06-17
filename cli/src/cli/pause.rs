use anyhow::Result;
use argh::FromArgs;
use probe_common::cli::ProbeCommand;

use super::usr1_handler;

/// Pause the target process and listen for remote connection
#[derive(FromArgs)]
#[argh(subcommand, name = "pause")]
pub struct PauseCommand {
    /// address to listen
    #[argh(option, short = 'a')]
    address: Option<String>,
}

impl PauseCommand {
    pub fn run(&self, pid: i32) -> Result<()> {
        let cmd = ProbeCommand::Pause {
            address: self.address.clone(),
        };
        let cmd = ron::to_string(&cmd)?;
        usr1_handler(cmd, pid)
    }
}
