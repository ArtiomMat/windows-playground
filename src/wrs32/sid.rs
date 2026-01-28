use std::mem;

use windows::Win32::Security::*;
use windows::core::*;

use super::*;

pub struct Sid(VarBuf<u8>);

impl Sid {
    /// Copies a raw `SID`.
    /// 
    /// # Panics if invalid.
    /// 
    /// If SID is invalid will panic.
    pub fn copy_raw(psid: PSID) -> Self {
        assert!(!psid.is_invalid());

        let size = unsafe { GetLengthSid(psid) } as usize;
        // SAFETY: This is safe because:
        //         1. 4-byte aligned like SIDs need to be.
        //         2. Size is in bytes and is given by GetLengthSid.
        //         3. And psid is asserted to be valid.
        let buffer =  unsafe {
            VarBuf::<u8>::from_aligned_raw(size, 4, psid.0 as *const u8)
        };

        Self(buffer)
    }

    pub fn lookup_local_account(&self) -> Result<String> {
        let mut cch_name: u32 = 0;
        let mut cch_domain_name: u32 = 0;
        let mut peuse: SID_NAME_USE = unsafe { mem::zeroed() };
        unsafe {
            LookupAccountSidW(
                None,
                PSID(self.0.as_c_void_ptr()),
                None,
                &mut cch_name,
                None,
                &mut cch_domain_name,
                &mut peuse,
            )
            .expect_err("Expected to fail here due to only querying");

            let mut name = vec![0u16; cch_name as usize];
            let mut domain_name = vec![0u16; cch_domain_name as usize];

            LookupAccountSidW(
                None,
                PSID(self.0.as_c_void_ptr()),
                Some(PWSTR::from_raw(name.as_mut_ptr())),
                &mut cch_name,
                Some(PWSTR::from_raw(domain_name.as_mut_ptr())),
                &mut cch_domain_name,
                &mut peuse,
            )?;

            Ok(PWSTR::from_raw(name.as_mut_ptr()).to_string()?)
        }
    }
}
