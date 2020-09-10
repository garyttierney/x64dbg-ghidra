use std::ffi::CString;
use x64dbg_bridge_sys::*;

pub fn plugin_print<S: AsRef<str>>(text: S) {
    let c_str = CString::new(text.as_ref()).unwrap();

    unsafe {
        plugin::_plugin_logprint(c_str.as_ptr());
    }
}
