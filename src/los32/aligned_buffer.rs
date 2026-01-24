use std::ops::Deref;
use std::{alloc, ffi, mem};

/// Similar to Box but much less safe because I am bad at rust.
pub struct AlignedBuffer<T> {
    pub ptr: *mut T,
    layout: alloc::Layout,
}

impl<T> AlignedBuffer<T> {
    /// Allocates a zeroed buffer that is not necessarily aligned to `T`,
    /// you choose.
    ///
    /// Don't forget to actually copy data here
    pub unsafe fn new_custom_aligned(size: usize, alignment: usize) -> Self {
        let layout = alloc::Layout::from_size_align(size, alignment).unwrap();
        let ptr = unsafe { alloc::alloc_zeroed(layout) as *mut T };
        Self { ptr, layout }
    }

    /// Allocates a zeroed buffer that is aligned to `T`.
    ///
    /// Don't forget to actually copy data here
    pub fn new(size: usize) -> Self {
        unsafe { Self::new_custom_aligned(size, mem::align_of::<T>()) }
    }

    pub fn as_c_void_ptr_mut(&self) -> *mut ffi::c_void {
        self.ptr as *mut ffi::c_void
    }

    pub fn as_c_void_ptr(&self) -> *const ffi::c_void {
        self.ptr as *const ffi::c_void
    }
}

impl<T> Deref for AlignedBuffer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> Drop for AlignedBuffer<T> {
    fn drop(&mut self) {
        unsafe { alloc::dealloc(self.ptr as *mut u8, self.layout) };
    }
}
