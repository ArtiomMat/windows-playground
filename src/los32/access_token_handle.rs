use std::{alloc, ffi};

use windows::Win32::Foundation::*;
use windows::Win32::Security::*;
use windows::core::*;

pub struct AccessTokenHandle(pub(super) HANDLE);

impl AccessTokenHandle {
    unsafe fn get_information<T>(
        &self,
        tokeninformationclass: TOKEN_INFORMATION_CLASS,
    ) -> Result<Box<T>> {
        // let mut x: T = unsafe { mem::zeroed() };
        let buffer_size;
        let mut needed_size: u32 = 0;
        let buffer = unsafe {
            let err = GetTokenInformation(self.0, tokeninformationclass, None, 0, &mut needed_size)
                .expect_err("Expected to have some error.");
            assert!(err.code() == ERROR_INSUFFICIENT_BUFFER.into());

            // Allocating the buffer necessary
            buffer_size = needed_size as usize;
            let layout = alloc::Layout::from_size_align(buffer_size, align_of::<T>()).unwrap();
            let buffer = alloc::alloc(layout);
            assert!(!buffer.is_null());

            GetTokenInformation(
                self.0,
                tokeninformationclass,
                Some(buffer as *mut ffi::c_void),
                buffer_size as u32,
                &mut needed_size,
            )?;

            buffer
        };

        if needed_size as usize != buffer_size {
            Err(unsafe { GetLastError() }.into())
        } else {
            // FIXME: Box<T> assumes the size is only T, but it may be more.
            let result = unsafe { Box::<T>::from_raw(buffer as *mut T) };
            Ok(result)
        }
    }

    pub fn get_owner(&self) -> Result<Box<TOKEN_OWNER>> {
        unsafe { self.get_information::<TOKEN_OWNER>(TokenOwner) }
    }
}

impl Drop for AccessTokenHandle {
    fn drop(&mut self) {
        unsafe { self.0.free() };
    }
}
