use std::ops::Deref;
use std::{alloc, ffi, mem, ptr};

/// Similar to Box but much less safe because I am bad at rust.
pub struct AlignedBuffer<T> {
    pub ptr: ptr::NonNull<T>,
    layout: alloc::Layout,
}

impl<T> AlignedBuffer<T> {
    /// Allocates a zeroed buffer that is not necessarily aligned to `T`,
    /// you choose.
    ///
    /// Don't forget to actually copy data here
    ///
    /// # Safety
    ///
    /// You are responsible for ensuring that alignment is not
    /// incorrect regarding `T`.
    pub unsafe fn new_custom_aligned(size: usize, alignment: usize) -> Self {
        let layout = alloc::Layout::from_size_align(size, alignment).unwrap();

        // SAFETY: This is indeed unsafe due to trusting caller on alignment.
        //         However, null is handled, that is guaranteed.
        let ptr = unsafe {
            ptr::NonNull::new(alloc::alloc(layout) as *mut T)
                .unwrap_or_else(|| alloc::handle_alloc_error(layout))
        };

        Self { ptr, layout }
    }

    /// Allocates a zeroed buffer that is aligned to `T`.
    ///
    /// Don't forget to actually copy data here
    ///
    /// # Panics on small size
    ///
    /// While the size is allowed to be bigger for any reason, it must not
    /// be smaller than T.
    pub fn new(size: usize) -> Self {
        assert!(size >= mem::size_of::<T>(), "Size must be >= T!");

        // SAFETY: This is safe because alignment is exact and size is
        //         asserted.
        unsafe { Self::new_custom_aligned(size, mem::align_of::<T>()) }
    }

    pub fn as_c_void_ptr_mut(&self) -> *mut ffi::c_void {
        self.ptr.as_ptr() as *mut ffi::c_void
    }

    pub fn as_c_void_ptr(&self) -> *const ffi::c_void {
        self.ptr.as_ptr() as *const ffi::c_void
    }
}

impl<T> Deref for AlignedBuffer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: This is safe because ptr is NonNull.
        //         However, one must assume that it is aligned.
        //         TODO: Is this ok?
        unsafe { &*(self.ptr.as_ptr()) }
    }
}

impl<T> Drop for AlignedBuffer<T> {
    fn drop(&mut self) {
        // SAFETY: This is safe because the exact same layout is used.
        //         And ptr was allocated with `alloc()` with this layout.
        //         Casting is safe because this is the original type.
        unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, self.layout) };
    }
}
