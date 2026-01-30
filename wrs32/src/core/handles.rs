use windows::Win32::Foundation::HANDLE;
use windows::Win32::Security::Authorization::SE_OBJECT_TYPE;

pub trait AsHandle {
    fn as_handle(&self) -> HANDLE;
}

pub trait TypedHandle: AsHandle {
    fn object_type(&self) -> SE_OBJECT_TYPE;
}
