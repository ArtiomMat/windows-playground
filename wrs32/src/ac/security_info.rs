use std::{ffi, mem, ptr};

use crate::core::TypedHandle;

use windows::Win32::Foundation::{ERROR_SUCCESS, HLOCAL, LocalFree};
use windows::Win32::Security::Authorization::GetSecurityInfo;
use windows::Win32::Security::*;
use windows::core::*;

#[derive(Debug)]
pub struct SecurityInfo {
    pub owner_sid: PSID,
    pub group_sid: PSID,
    pub dacl: *mut ACL,
    pub sacl: *mut ACL,
    ptr: PSECURITY_DESCRIPTOR,
}

impl Drop for SecurityInfo {
    fn drop(&mut self) {
        _ = unsafe { LocalFree(Some(HLOCAL(self.ptr.0 as *mut ffi::c_void))) };
    }
}

pub trait SecurityInfoFetcher: TypedHandle {
    fn get_security_info(&self) -> Result<SecurityInfo> {
        let mut owner_sid: PSID = PSID(ptr::null_mut());
        let mut group_sid: PSID = PSID(ptr::null_mut());
        let mut dacl: *mut ACL = ptr::null_mut();
        let mut sacl: *mut ACL = ptr::null_mut();

        // SAFETY: This is safe because it will be overriden below.
        let mut ptr: PSECURITY_DESCRIPTOR = unsafe { mem::zeroed() };

        // SAFETY: This is safe because 
        let error = unsafe {
            GetSecurityInfo(
                self.as_handle(),
                self.object_type(),
                OWNER_SECURITY_INFORMATION
                    | GROUP_SECURITY_INFORMATION
                    | DACL_SECURITY_INFORMATION
                    | SACL_SECURITY_INFORMATION,
                Some(&mut owner_sid),
                Some(&mut group_sid),
                Some(&mut dacl),
                Some(&mut sacl),
                Some(&mut ptr),
            )
        };

        if error != ERROR_SUCCESS {
            return Err(error.into());
        }

        Ok(SecurityInfo{
            owner_sid,
            group_sid,
            dacl,
            sacl,
            ptr
        })
    }
}

