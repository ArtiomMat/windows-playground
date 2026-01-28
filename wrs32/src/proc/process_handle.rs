use windows::Win32::Foundation::*;
use windows::Win32::Security::Authorization::SE_KERNEL_OBJECT;
use windows::Win32::Security::*;
use windows::Win32::System::Threading::*;
use windows::core::*;

use crate::core::{AsHandle, TypedHandle};

use crate::ac::{AccessTokenHandle, SecurityInfoFetcher};

pub struct ProcessHandle(HANDLE);

impl ProcessHandle {
    pub fn get_current() -> Self {
        // SAFETY: This is safe because GetCurrentProcess() is guaranteed to
        //         return a valid handle.
        Self(unsafe { GetCurrentProcess() })
    }

    pub fn open(
        dwdesiredaccess: PROCESS_ACCESS_RIGHTS,
        binherithandle: bool,
        dwprocessid: u32,
    ) -> Result<Self> {
        // SAFETY: This is safe because we handle the case of an error.
        //         None of the parameters can cause UB, only errors.
        unsafe {
            let h = OpenProcess(dwdesiredaccess, binherithandle, dwprocessid)?;
            Ok(Self(h))
        }
    }

    pub fn open_token(&self, desiredaccess: TOKEN_ACCESS_MASK) -> Result<AccessTokenHandle> {
        let mut raw_access_token_handle: HANDLE = INVALID_HANDLE_VALUE;
        unsafe {
            OpenProcessToken(self.0, desiredaccess, &mut raw_access_token_handle)?;
        }
        if raw_access_token_handle.is_invalid() {
            Err(unsafe { GetLastError() }.into())
        } else {
            Ok(AccessTokenHandle(raw_access_token_handle))
        }
    }
}

impl Drop for ProcessHandle {
    fn drop(&mut self) {
        // SAFETY: This is safe because free() simply does CloseHandle().
        unsafe { self.0.free() };
    }
}

impl AsHandle for ProcessHandle {
    fn as_handle(&self) -> HANDLE {
        self.0
    }
}

impl TypedHandle for ProcessHandle {
    fn object_type(&self) -> Authorization::SE_OBJECT_TYPE {
        SE_KERNEL_OBJECT
    }
}

impl SecurityInfoFetcher for ProcessHandle {}
