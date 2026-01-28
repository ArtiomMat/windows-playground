use std::ops::Deref;
use std::{alloc, ffi, mem, ptr};

/// Similar to `Box<T>` but easier to shoot yourself in the foot.
///
/// # `Buf<T> vs `Box<T>`
///
/// Primary reason to prefer it over `Box<T>` is because it allows for
/// arbitrary sizes and alignment, regardless of `T` itself.
pub struct Buf<T> {
    ptr: ptr::NonNull<T>,
    layout: alloc::Layout,
}

impl<T> Buf<T> {
    /// Allocates a zeroed buffer that is not necessarily aligned to `T`,
    /// you choose.
    ///
    /// Don't forget to actually copy data.
    ///
    /// # Panics
    ///
    /// - If alignment is not a multiple of `T`.
    /// - If size is not more or equal to `T`.
    pub fn new_custom_aligned(size: usize, alignment: usize) -> Self {
        assert!(size >= mem::size_of::<T>(), "Size must be >= T!");
        assert!(
            alignment % mem::align_of::<T>() == 0,
            "Alignment must be a multiple of T"
        );

        let layout = alloc::Layout::from_size_align(size, alignment).unwrap();

        // SAFETY: This is safe because:
        //         1. T can fit in the allocated buffer.
        //         2. The buffer is aligned to T.
        //         3. Allocation erros are handled.
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
    /// # Panics
    ///
    /// See [Self::new_custom_aligned].
    pub fn new(size: usize) -> Self {
        Self::new_custom_aligned(size, mem::align_of::<T>())
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    pub fn as_c_void_ptr(&self) -> *mut ffi::c_void {
        self.ptr.as_ptr() as *mut ffi::c_void
    }
}

impl<T> Deref for Buf<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: This is safe because ptr is NonNull.
        //         Guaranteed to be aligned.
        unsafe { &*(self.ptr.as_ptr()) }
    }
}

impl<T> Drop for Buf<T> {
    fn drop(&mut self) {
        // SAFETY: This is safe because the exact same layout is used.
        //         And ptr was allocated with `alloc()` with this layout.
        //         Casting is safe because this is the original type.
        unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, self.layout) };
    }
}

