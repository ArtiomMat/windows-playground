use std::ops::{Deref, DerefMut};
use std::{alloc, ffi, mem, ptr};

/// Similar to `Box<T>` but easier to shoot yourself in the foot.
///
/// For any meaningful operation, `T` must be sized.
///
/// # `Buf<T> vs `Box<T>`
///
/// Primary reason to prefer it over `Box<T>` is because it allows for
/// arbitrary sizes and alignment, regardless of `T` itself.
pub struct Buf<T> {
    ptr: ptr::NonNull<T>,
    layout: alloc::Layout,
}

impl<T> Buf<T>
where
    T: Sized,
{
    /// Allocates a buffer that is not necessarily aligned to `T`,
    /// you choose. The buffer is reset to `value`.
    ///
    /// # The rest of the buffer
    ///
    /// After the `T` part, the rest of the buffer is in an undefined state.
    /// You cannot directly access it via the given API, but if you choose to,
    /// you must ensure that it is initialized by you.
    ///
    /// # Panics
    ///
    /// - If alignment is not a multiple of `T` or a power of 2.
    /// - If size is not more or equal to `T`.
    pub fn new_aligned(size: usize, alignment: usize, value: T) -> Self {
        assert!(size >= mem::size_of::<T>(), "Size must be >= T!");
        assert!(
            alignment.is_multiple_of(mem::align_of::<T>()),
            "Alignment must be a multiple of T"
        );
        assert!(
            alignment.is_power_of_two(),
            "Alignment must be a power of 2"
        );

        let layout = alloc::Layout::from_size_align(size, alignment).unwrap();

        // SAFETY: This is safe because:
        //         1. `T` can fit in the allocated buffer.
        //         2. The buffer is aligned to `T`.
        //         3. Allocation erros are handled.
        let mut ptr = unsafe {
            ptr::NonNull::new(alloc::alloc(layout) as *mut T)
                .unwrap_or_else(|| alloc::handle_alloc_error(layout))
        };

        // SAFETY: This is safe because `value` is valid and `ptr` is both
        //         aligned and non-null.
        unsafe {
            ptr::write(ptr.as_mut(), value);
        }

        Self { ptr, layout }
    }

    /// See [Self::new_custom_aligned], but alignment is just native `T`.
    pub fn new(size: usize, value: T) -> Self {
        Self::new_aligned(size, mem::align_of::<T>(), value)
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

impl<T> DerefMut for Buf<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: This is safe because ptr is NonNull.
        //         Guaranteed to be aligned.
        unsafe { &mut *(self.ptr.as_ptr()) }
    }
}

impl<T> Drop for Buf<T> {
    fn drop(&mut self) {
        // SAFETY: This is safe because the exact same layout is used.
        //         And ptr was allocated with `alloc()` with this layout.
        //         Casting is safe because this is the original type.
        //         T is initialized, hence a drop_in_place() is correct.
        unsafe {
            ptr::drop_in_place(self.ptr.as_ptr());
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, self.layout);
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct X {
        a: i32,
        b: String,
        c: Box<i32>,
    }

    impl Default for X {
        fn default() -> Self {
            Self {
                a: Default::default(),
                b: Default::default(),
                c: Default::default(),
            }
        }
    }

    fn create_some_x() -> X {
        X {
            a: 123,
            b: "Hello".into(),
            c: Box::new(67),
        }
    }

    #[test]
    #[should_panic(expected = "Size must be >= T!")]
    fn sanity_value_panic_size() {
        _ = Buf::<X>::new_aligned(1, mem::align_of::<X>(), X::default());
    }

    #[test]
    #[should_panic(expected = "Alignment must be a multiple of T")]
    fn sanity_value_panic_align() {
        _ = Buf::<X>::new_aligned(mem::size_of::<X>(), 3, X::default());
    }

    #[test]
    fn sanity_value() {
        for size in [
            mem::size_of::<X>(),
            mem::size_of::<X>() * 3,
            mem::size_of::<X>() + 3,
        ] {
            let mut x = Buf::<X>::new_aligned(size, mem::align_of::<X>(), create_some_x());
            x.a = 55;
            assert!(x.a == 55);
            assert!(x.b == "Hello");
            assert!(*x.c == 67);
        }
    }
}
