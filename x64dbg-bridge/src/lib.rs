use std::error::Error;
use std::ffi::CString;
use std::os::raw::c_int;

use x64dbg_bridge_sys::plugin::*;

pub mod debug;
pub mod util;

#[derive(Debug)]
pub enum X64DbgError {
    InternalError,
}

impl Error for X64DbgError {}

impl std::fmt::Display for X64DbgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperErrorSideKick is here!")
    }
}

#[repr(transparent)]
pub struct PauseDebugEvent(PLUG_CB_PAUSEDEBUG);

#[repr(transparent)]
pub struct ResumeDebugEvent(PLUG_CB_RESUMEDEBUG);

#[repr(transparent)]
pub struct PluginInit(PLUG_INITSTRUCT);

#[derive(Debug)]
#[repr(transparent)]
pub struct PluginHandle(c_int);

impl PluginInit {
    pub fn initialize(&mut self) {
        self.0.sdkVersion = PLUG_SDKVERSION as i32;
    }

    pub fn name<S: AsRef<str>>(&mut self, name: S) {
        let c_name = CString::new(name.as_ref()).unwrap();
        let c_name_data = c_name.as_bytes_with_nul();

        self.0.pluginName[..c_name_data.len()]
            .copy_from_slice(unsafe { &*(c_name_data as *const [u8] as *const _) });
    }

    pub fn handle(&self) -> PluginHandle {
        PluginHandle(self.0.pluginHandle)
    }
}
