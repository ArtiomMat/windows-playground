
use windows::Win32::Foundation::*;
use windows::Win32::Security::*;
use windows::Win32::Security::Authorization::*;
use windows::core::*;

pub trait AsHandle {
    fn as_handle(&self) -> HANDLE;
}

pub trait TypedHandle: AsHandle {
    fn object_type(&self) -> SE_OBJECT_TYPE;
}
