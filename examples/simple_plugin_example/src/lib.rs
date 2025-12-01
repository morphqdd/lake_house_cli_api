use clap::{Command, arg};
use lake_house_cli_api::api::Plugin;

pub struct SimplePlugin;

impl Plugin for SimplePlugin {
    fn name(&self) -> &'static str {
        "say"
    }

    fn command(&self) -> clap::Command {
        Command::new(self.name())
            .about("Just saying something")
            .arg(arg!(<SOMETHING>))
    }

    fn run(&self, matches: clap::ArgMatches) -> anyhow::Result<()> {
        println!(
            "{}",
            matches
                .get_one::<String>("SOMETHING")
                .ok_or(anyhow::anyhow!("SOMETHING not found!"))?
        );
        Ok(())
    }
}

#[unsafe(no_mangle)]
extern "C" fn plugin() -> *mut SimplePlugin {
    Box::leak(Box::new(SimplePlugin)) as *mut _
}
