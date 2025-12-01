use std::ffi::{c_char, c_void};

use clap::{ArgMatches, Command, arg};
use lake_house_cli_api::api::{Arg, Plugin, PluginApi, PluginCommand};

pub struct SimplePlugin;

impl SimplePlugin {
    fn _name(&self) -> &'static str {
        "say"
    }

    fn _command(&self) -> PluginCommand {
        PluginCommand {
            name: self._name(),
            about: "Just say something",
            args: vec![Arg {
                name: "something",
                ..Default::default()
            }],
            subcommands: vec![],
        }
    }

    fn _run(&self, matches: clap::ArgMatches) -> anyhow::Result<()> {
        println!(
            "{}",
            matches
                .get_one::<String>("SOMETHING")
                .ok_or(anyhow::anyhow!("SOMETHING not found!"))?
        );
        Ok(())
    }
}

impl Plugin for SimplePlugin {
    fn name(&self) -> &'static str {
        "say"
    }

    fn command(&self) -> Command {
        Command::new(self.name())
            .about("Just say something")
            .arg(arg!(<SOME>))
    }

    fn run(&self, matches: ArgMatches) -> anyhow::Result<()> {
        println!(
            "{}",
            matches
                .get_one::<String>("SOMETHING")
                .ok_or(anyhow::anyhow!("SOMETHING not found!"))?
        );
        Ok(())
    }
}

impl SimplePlugin {
    extern "C" fn instantiate() -> *mut c_void {
        Box::into_raw(Box::new(SimplePlugin)) as *mut c_void
    }

    extern "C" fn name(ptr: *mut c_void) -> *const c_char {
        let this = unsafe { &*(ptr as *mut SimplePlugin) };
        std::ffi::CString::new(this._name()).unwrap().into_raw()
    }

    extern "C" fn command(ptr: *mut c_void) -> *mut PluginCommand {
        let this = unsafe { &*(ptr as *mut SimplePlugin) };
        Box::into_raw(Box::new(this._command()))
    }

    extern "C" fn run(ptr: *mut c_void, matches: *const ArgMatches) -> i32 {
        let this = unsafe { &*(ptr as *mut SimplePlugin) };
        if this._run(unsafe { (*matches).clone() }).is_ok() {
            0
        } else {
            1
        }
    }

    extern "C" fn drop(ptr: *mut c_void) {
        unsafe { drop(Box::from_raw(ptr as *mut SimplePlugin)) };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn plugin() -> *const PluginApi {
    &PluginApi {
        instantiate: SimplePlugin::instantiate,
        name: SimplePlugin::name,
        command: SimplePlugin::command,
        run: SimplePlugin::run,
        drop: SimplePlugin::drop,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dyn_plugin() -> *const SimplePlugin {
    &SimplePlugin
}
