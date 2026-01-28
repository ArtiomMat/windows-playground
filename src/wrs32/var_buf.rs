use std::ops::{Index, IndexMut};

use std::{alloc, ffi, mem, ptr};

use super::buf::Buf;

pub struct VarBuf<T> {
    buf: Buf<T>,
    count: usize,
}

impl<T> VarBuf<T> {
    pub fn new_aligned(count: usize, alignment: usize, value: T) -> Self
    where
        T: Clone,
    {
        let buf = Buf::new_aligned(count * mem::size_of::<T>(), alignment, value.clone());

        // SAFETY: This is safe beause:
        //         1. The size is ensured to be `count * T`.
        //         2. The alignment is asserted to be valid inside of `Buf`.
        //         3. All values are initialized to `value`([0] initialized
        //         above).
        unsafe {
            for i in 1..count {
                *buf.as_ptr().add(i) = value.clone();
            }
        }

        Self { buf, count }
    }

    pub unsafe fn from_aligned_raw(count: usize, alignment: usize, ptr: *const T) -> Self {
        // TODO: add SAFETY notes

        let buf = unsafe {
            Buf::<T>::new_aligned(count * mem::size_of::<T>(), alignment, std::mem::zeroed())
        };

        unsafe {
            std::ptr::copy_nonoverlapping(ptr, buf.as_ptr(), count);
        }

        Self { buf, count }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.buf.as_ptr()
    }

    pub fn as_c_void_ptr(&self) -> *mut ffi::c_void {
        self.buf.as_ptr() as *mut ffi::c_void
    }
}

impl<T> Index<usize> for VarBuf<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.count);
        // SAFETY: This is safe because:
        //         1. The pointer aligned.
        //         2. All elements are initialized.
        //         3. The index is asserted to be in range.
        unsafe { &*self.buf.as_ptr().add(index) }
    }
}

impl<T> IndexMut<usize> for VarBuf<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.count);
        // SAFETY: This is safe because:
        //         1. The pointer aligned.
        //         2. All elements are initialized.
        //         3. The index is asserted to be in range.
        unsafe { &mut *self.buf.as_ptr().add(index) }
    }
}
