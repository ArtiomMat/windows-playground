use std::{alloc, ffi, mem, ptr};

use windows::Win32::Foundation::*;
use windows::Win32::Security::*;
use windows::core::*;

pub struct AccessTokenHandle(pub(super) HANDLE);

impl AccessTokenHandle {
    /// Returns a Box(subject to change due to UB, see FIXME), which has the
    /// requested information struct pointer, its size is not necessarily T, as
    /// windows allocates data pointed to by the struct right after it.
    /// 
    /// # Panics on `T` and class mismatch
    ///
    /// If `T` and `tokeninformationclass` are mismatched the function will
    /// panic.
    unsafe fn get_information<T>(
        &self,
        tokeninformationclass: TOKEN_INFORMATION_CLASS,
    ) -> Result<Box<T>> {
        let mut needed_size: u32 = 0;
        // SAFETY: Calling GetTokenInformation() with a null buffer and size=0
        //         is the documented way to query the required buffer size. The 
        //         function writes only to `needed_size`, which is a valid, 
        //         initialized out-parameter. It is asserted that there is both 
        //         the right error and size of the buffer.
        unsafe {
            let err = GetTokenInformation(self.0, tokeninformationclass, None, 0, &mut needed_size)
                .err()
                .unwrap_or(ERROR_SUCCESS.into());
            assert!(
                err == ERROR_INSUFFICIENT_BUFFER.into(),
                "Error must be insufficient buffer"
            );
        }

        assert!(
            needed_size as usize >= mem::size_of::<T>(),
            "Needed size is too small for T, most likely a mismatch between T and the class given"
        );

        let allocated_size = needed_size as usize;
        let layout = alloc::Layout::from_size_align(allocated_size, mem::align_of::<T>()).unwrap();
        // SAFETY: Allocation is done with alignment of T alongside a checked
        //         size for the buffer.
        let info_buffer = unsafe {
            ptr::NonNull::new(alloc::alloc(layout))
                .unwrap_or_else(|| alloc::handle_alloc_error(layout))
        };

        // SAFETY: This is safe because:
        //         1. The handle(self.0) is guaranteed to stay valid until the
        //         drop.
        //         2. info_buffer is non-null and allocated_size reflects its 
        //         size hence in the worst case we should get 
        //         ERROR_INSUFFICIENT_BUFFER.
        //         3. needed size is once again queried for later querying and 
        //         is valid, being only an out parameter.
        unsafe {
            GetTokenInformation(
                self.0,
                tokeninformationclass,
                Some(info_buffer.as_ptr() as *mut ffi::c_void),
                allocated_size as u32,
                &mut needed_size,
            )?;
        }

        if needed_size as usize != allocated_size {
            Err(unsafe { GetLastError() }.into())
        } else {
            // FIXME: Box<T> assumes the size is only T, but it may be more.
            let result = unsafe { Box::<T>::from_raw(info_buffer.as_ptr() as *mut T) };
            Ok(result)
        }
    }

    pub fn get_owner(&self) -> Result<Box<TOKEN_OWNER>> {
        // SAFETY: TOKEN_OWNER type is expected given TokenOwner class.
        unsafe { self.get_information::<TOKEN_OWNER>(TokenOwner) }
    }
}

impl Drop for AccessTokenHandle {
    fn drop(&mut self) {
        unsafe { self.0.free() };
    }
}
