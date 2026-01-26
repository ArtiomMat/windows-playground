use super::{Sid, TypedHandle};

use windows::Win32::Foundation::{ERROR_SUCCESS, GetLastError, LocalFree};
use windows::Win32::Security::Authorization::GetSecurityInfo;
use windows::Win32::Security::*;
use windows::core::*;

struct SecurityInfo {
    pub owner_sid: Sid,
    pub group_sid: Sid,
    pub dacl: ACL,
    pub sacl: ACL,
    internal: OBJECT_SECURITY_INFORMATION,
}

impl Drop for SecurityInfo {
    fn drop(&mut self) {
        
    }
}

trait SecurityInfoFetcher: TypedHandle {
    fn get_security_info(&self) -> Result<SecurityInfo> {
        unsafe {
            let error = GetSecurityInfo(
                self.as_handle(),
                self.object_type(),
                OBJECT_SECURITY_INFORMATION(0),
                None,
                None,
                None,
                None,
                None,
            );

            if error != ERROR_SUCCESS {
                return Err(GetLastError().into());
            }
        }

        Ok(())
    }
}

// impl<T> SecurityInfoFetcher for T where T: TypedHandle {

// }
