use crate::db::Db;
use core::panic;
use std::{
    ffi::{c_char, c_int, c_void, CString},
    i8,
    mem::MaybeUninit,
};
mod sys;

const HELLO_OPER: sys::fuse_operations = generate_fuse_ops();

#[derive(Debug)]
struct FuseClient {
    db: Db,
}

unsafe extern "C" fn fuse_client_getattr(path: *const c_char, statbuf: *mut sys::stat) -> c_int {
    println!("Hello from fuse client getattr");
    (*statbuf).st_mode = sys::S_IFDIR | 0o755;

    0
}

const fn generate_fuse_ops() -> sys::fuse_operations {
    unsafe {
        let mut ops: sys::fuse_operations = MaybeUninit::zeroed().assume_init();
        ops.getattr = Some(fuse_client_getattr);
        ops
    }
}

pub fn run_fuse_client(db: Db) {
    let mut client = FuseClient { db };
    // argc and argv need compatible string hence we use CString
    let args: Vec<CString> = std::env::args().map(|s| CString::new(s).unwrap()).collect();

    let mut args: Vec<*mut i8> = args.into_iter().map(|s| s.into_raw()).collect();

    let mut args = sys::fuse_args {
        argc: args.len().try_into().unwrap(),
        argv: args.as_mut_ptr(),
        allocated: 0,
    };

    unsafe {
        let ret = sys::fuse_opt_parse(&mut args, std::ptr::null_mut(), std::ptr::null_mut(), None);
        if ret == -1 {
            panic!("Failed to parse fuse args")
        }

        sys::fuse_main_real(
            args.argc,
            args.argv,
            &HELLO_OPER,
            std::mem::size_of_val(&HELLO_OPER),
            &mut client as *mut FuseClient as *mut c_void,
        );
    }
}
