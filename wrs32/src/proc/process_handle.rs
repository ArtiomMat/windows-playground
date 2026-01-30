use windows::Win32::Foundation::{GetLastError, HANDLE, INVALID_HANDLE_VALUE};
use windows::Win32::Security::Authorization::{SE_KERNEL_OBJECT, SE_OBJECT_TYPE};
use windows::Win32::Security::TOKEN_ACCESS_MASK;
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcess, OpenProcessToken, PROCESS_ACCESS_RIGHTS};
use windows::core::{Free, Result};

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
        desired_access: PROCESS_ACCESS_RIGHTS,
        b_inherit_handle: bool,
        pid: u32,
    ) -> Result<Self> {
        // SAFETY: This is safe because we handle the case of an error.
        //         None of the parameters can cause UB, only errors.
        unsafe {
            let h = OpenProcess(desired_access, b_inherit_handle, pid)?;
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
    fn object_type(&self) -> SE_OBJECT_TYPE {
        SE_KERNEL_OBJECT
    }
}

impl SecurityInfoFetcher for ProcessHandle {}
