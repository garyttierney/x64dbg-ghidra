use crate::X64DbgError;
use std::fmt;
use std::fmt::{Debug, Formatter};
use x64dbg_bridge_sys::bridge::*;

#[repr(transparent)]
pub struct DebugFunctions(DBGFUNCTIONS);

#[repr(transparent)]
pub struct RegisterDump(REGDUMP);

impl Debug for RegisterDump {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rip = {}, rax = {}, rcx = {}",
            self.0.regcontext.cip, self.0.regcontext.cax, self.0.regcontext.ccx,
        )
    }
}

pub fn register_dump() -> Result<RegisterDump, X64DbgError> {
    let regdump = unsafe {
        let mut regdump = std::mem::zeroed();
        let success = DbgGetRegDumpEx(
            &mut regdump as *mut REGDUMP,
            std::mem::size_of::<REGDUMP>() as u64,
        );

        if !success {
            return Err(X64DbgError::InternalError);
        }

        regdump
    };

    Ok(RegisterDump(regdump))
}
