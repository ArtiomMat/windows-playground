use std::mem;
use std::ptr;

use windows::Win32::Foundation::*;
use windows::Win32::Security::*;
use windows::Win32::System::Threading::*;
use windows::core::*;

use super::aligned_buffer::*;

pub struct Sid(AlignedBuffer<u8>);

impl Sid {
    pub fn copy_raw(psid: PSID) -> Self {
        let size = unsafe { GetLengthSid(psid) } as usize;
        // SIDs should be 4-byte aligned
        let buffer = unsafe { AlignedBuffer::<u8>::new_custom_aligned(size, 4) };

        unsafe {
            std::ptr::copy_nonoverlapping(psid.0 as *const u8, buffer.ptr, size);
        }

        Self(buffer)
    }

    pub fn lookup_local_account_sid(&self) -> Result<String> {
        let mut cch_name: u32 = 0;
        let mut cch_domain_name: u32 = 0;
        let mut peuse: SID_NAME_USE = unsafe { mem::zeroed() };
        unsafe {
            LookupAccountSidW(
                None,
                PSID(self.0.as_c_void_ptr_mut()),
                None,
                &mut cch_name,
                None,
                &mut cch_domain_name,
                &mut peuse,
            )
            .expect_err("Expected to fail here due to only querying");

            let name = AlignedBuffer::new(cch_name as usize);
            let domain_name = AlignedBuffer::new(cch_domain_name as usize);

            LookupAccountSidW(
                None,
                PSID(self.0.as_c_void_ptr_mut()),
                Some(PWSTR::from_raw(name.ptr)),
                &mut cch_name,
                Some(PWSTR::from_raw(domain_name.ptr)),
                &mut cch_domain_name,
                &mut peuse,
            )?;

            Ok(PWSTR::from_raw(name.ptr).to_string()?)
        }
    }
}
