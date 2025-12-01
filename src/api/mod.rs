use std::ffi::{c_char, c_void};

use anyhow::Result;
use clap::{ArgMatches, Command};

#[repr(C)]
pub struct PluginApi {
    pub instantiate: extern "C" fn() -> *mut std::ffi::c_void,
    pub name: extern "C" fn(*mut std::ffi::c_void) -> *const c_char,
    pub command: extern "C" fn(*mut std::ffi::c_void) -> *mut Command,
    pub run: extern "C" fn(*mut std::ffi::c_void, *const ArgMatches) -> Result<(), ApiError>,
    pub drop: extern "C" fn(*mut std::ffi::c_void),
}

pub struct ApiError(String);

impl ApiError {
    pub fn new(msg: &str) -> Self {
        Self(msg.into())
    }
}
