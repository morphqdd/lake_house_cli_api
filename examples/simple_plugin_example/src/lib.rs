use std::ffi::{c_char, c_void};

use anyhow::{Error, bail};
use clap::{ArgMatches, Command, arg};
use lake_house_cli_api::api::{ApiError, PluginApi};

pub struct SimplePlugin;

impl SimplePlugin {
    fn _name(&self) -> &'static str {
        "say"
    }

    fn _command(&self) -> Command {
        Command::new(self._name())
            .about("Just say something")
            .arg(arg!(<SOMETHING>))
    }

    fn _run(&self, matches: clap::ArgMatches) -> Result<(), ApiError> {
        println!(
            "{}",
            matches
                .get_one::<String>("SOMETHING")
                .ok_or(ApiError::new("SOMETHING not found!"))?
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

    extern "C" fn command(ptr: *mut c_void) -> *mut Command {
        let this = unsafe { &*(ptr as *mut SimplePlugin) };
        Box::into_raw(Box::new(this._command()))
    }

    extern "C" fn run(ptr: *mut c_void, matches: *const ArgMatches) -> Result<(), ApiError> {
        let this = unsafe { &*(ptr as *mut SimplePlugin) };
        if let Some(matches) = unsafe { matches.as_ref() } {
            return this._run(matches.clone());
        }
        Err(ApiError::new("Matches not found!"))
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
