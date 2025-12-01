use std::ffi::{c_char, c_void};

use anyhow::Result;
use clap::{ArgMatches, Command};

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn command(&self) -> Command;
    fn run(&self, matches: ArgMatches) -> Result<()>;
}

#[repr(C)]
pub struct PluginApi {
    pub instantiate: extern "C" fn() -> *mut std::ffi::c_void,
    pub name: extern "C" fn(*mut std::ffi::c_void) -> *const c_char,
    pub command: extern "C" fn(*mut std::ffi::c_void) -> *mut c_void,
    pub run: extern "C" fn(*mut std::ffi::c_void, *const ArgMatches) -> i32,
    pub drop: extern "C" fn(*mut std::ffi::c_void),
}
