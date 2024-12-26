mod commands;
mod config;
mod tools;
mod engines;

use crate::config::{ProjectOptions, RunOptions, RunSubCommand, ToolOptions};
use structopt::StructOpt;

fn main() {
    let tool = config::ToolOptions::from_file();
    let project = config::ProjectOptions::from_file();
    let run = config::RunOptions::from_args();

    let run = RipleyRun::build(tool, project, run);

    match run.command {
        SubCommand::Pull => commands::pull::execute(),
        SubCommand::Reset => commands::reset::execute(),
        SubCommand::Create { name } => commands::create::execute(&name),
    }
}

#[derive(Debug)]
pub struct RipleyRun {
    pub database: String,
    pub command: SubCommand,
}

impl RipleyRun {
    pub fn build(
        tool: Option<ToolOptions>,
        project: Option<ProjectOptions>,
        run: RunOptions,
    ) -> RipleyRun {
        RipleyRun {
            database: "".to_string(),
            command: match run.cmd {
                None => SubCommand::Reset,
                Some(c) => match c {
                    RunSubCommand::Pull => SubCommand::Pull,
                    RunSubCommand::Reset => SubCommand::Reset,
                    RunSubCommand::Create { name } => SubCommand::Create { name },
                },
            },
        }
    }
}

#[derive(Debug)]
pub enum SubCommand {
    Create { name: String },
    Pull,
    Reset,
}
