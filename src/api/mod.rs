use anyhow::Result;
use clap::{ArgMatches, Command};

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn command(&self) -> Command;
    fn run(&self, matches: ArgMatches) -> Result<()>;
}
