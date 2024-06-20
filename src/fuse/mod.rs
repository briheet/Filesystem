use std::{ffi::CString, i8};

mod sys;

pub fn new_fuse_client() {
    // argc and argv need compatible string hence we use CString
    let args: Vec<CString> = std::env::args().map(|s| CString::new(s).unwrap()).collect();

    let mut args: Vec<*mut i8> = args.into_iter().map(|s| s.into_raw()).collect();

    let args = sys::fuse_args {
        argc: args.len().try_into().unwrap(),
        argv: args.as_mut_ptr(),
        allocated: 0,
    };
}
