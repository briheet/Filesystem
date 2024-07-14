#[warn(unused_imports)]
use crate::db::{Db, DbItem};
use core::panic;

#[warn(unused_imports)]
use rusqlite::named_params;
use std::{
    ffi::{c_char, c_int, c_void, CStr, CString},
    fs, i8,
    mem::MaybeUninit,
    path::{Path, PathBuf},
};
mod sys;

const HELLO_OPER: sys::fuse_operations = generate_fuse_ops();

macro_rules! return_errno {
    () => {
        return -std::io::Error::last_os_error().raw_os_error().unwrap();
    };
}

#[derive(Debug)]
struct FuseClient {
    db: Db,
}

#[derive(Debug)]
enum FuseClientPath {
    Root,
    ById,
    DbPath(PathBuf),
    Unknown,
}

unsafe fn c_to_rust_path(s: *const c_char) -> &'static Path {
    Path::new(CStr::from_ptr(s).to_str().unwrap())
}

unsafe fn rust_to_c_path(p: &Path) -> CString {
    CString::new(p.to_path_buf().into_os_string().into_encoded_bytes()).unwrap()
}

unsafe fn get_client() -> &'static mut FuseClient {
    let context = sys::fuse_get_context();
    let client = (*context).private_data as *mut FuseClient;
    &mut *client
}

impl FuseClient {
    fn parse_path(&self, path: &Path) -> FuseClientPath {
        if path == Path::new("/") {
            FuseClientPath::Root
        } else if path == Path::new("/by-id") {
            FuseClientPath::ById
        } else if let Ok(v) = path.strip_prefix("/by-id") {
            return FuseClientPath::DbPath(self.db.fs_root().canonicalize().unwrap().join(v));
        } else {
            println!("Unhandled path: {:?}", path);
            return FuseClientPath::Unknown;
        }
    }
}

#[warn(unused_variables)]
unsafe extern "C" fn fuse_client_getattr(path: *const c_char, statbuf: *mut sys::stat) -> c_int {
    let client = get_client();
    let parsed_path = client.parse_path(c_to_rust_path(path));

    match parsed_path {
        FuseClientPath::DbPath(p) => {
            let p_cstring = CString::new(p.into_os_string().into_encoded_bytes()).unwrap();
            let ret = sys::lstat(p_cstring.as_ptr(), statbuf);
            if ret == -1 {
                return_errno!()
            }
            return ret;
        }
        _ => {
            (*statbuf).st_mode = sys::S_IFDIR | 0o755;
        }
    }

    0
}

unsafe extern "C" fn fuse_client_readdir(
    path: *const c_char,
    buf: *mut c_void,
    mut filler: sys::fuse_fill_dir_t,
    _offset: sys::off_t,
    _info: *mut sys::fuse_file_info,
) -> c_int {
    // get our database

    let client = get_client();
    let parsed_path = client.parse_path(c_to_rust_path(path));

    let filler = filler.as_mut().unwrap();
    match parsed_path {
        FuseClientPath::Root => {
            let by_id_folder = CString::new("by-id").unwrap();
            filler(buf, by_id_folder.as_ptr(), std::ptr::null(), 0);
        }
        FuseClientPath::ById => {
            for item in client.db.iterate_items() {
                let name = CString::new(item.id.0.to_string()).unwrap();
                filler(buf, name.as_ptr(), std::ptr::null(), 0);
            }
        }
        FuseClientPath::DbPath(p) => {
            for item in fs::read_dir(p).unwrap() {
                let item = item.unwrap();
                let name = CString::new(item.file_name().into_encoded_bytes()).unwrap();
                filler(buf, name.as_ptr(), std::ptr::null(), 0);
            }
        }
        p => {
            println!("Unhandled path: {:?}", p)
        }
    }
    0
}

unsafe extern "C" fn fuse_client_open(
    path: *const c_char,
    info: *mut sys::fuse_file_info,
) -> c_int {
    let client = get_client();
    let mapped_path = client.parse_path(c_to_rust_path(path));

    if let FuseClientPath::DbPath(p) = mapped_path {
        let ret = sys::open(rust_to_c_path(&p).as_ptr(), (*info).flags);
        if ret == -1 {
            return_errno!()
        }
        (*info).fh = ret.try_into().unwrap();
        return 0;
    }
    println!("Hello from fuse clinet open {:?}", mapped_path);
    0
}

unsafe extern "C" fn fuse_client_create(
    path: *const c_char,
    mode: sys::mode_t,
    info: *mut sys::fuse_file_info,
) -> c_int {
    let client = get_client();
    let mapped_path = client.parse_path(c_to_rust_path(path));

    println!("mapped_path in create: {:?}", mapped_path);
    if let FuseClientPath::DbPath(p) = mapped_path {
        let ret = sys::open(rust_to_c_path(&p).as_ptr(), (*info).flags, mode);
        if ret == -1 {
            return_errno!()
        }
    }

    -1
}

unsafe extern "C" fn fuse_client_chmod(arg1: *const c_char, arg2: sys::mode_t) -> c_int {
    println!("unimplemented chmod");
    0
}

unsafe extern "C" fn fuse_client_chown(
    path: *const c_char,
    arg2: sys::uid_t,
    arg3: sys::gid_t,
) -> c_int {
    println!("unimplemented chown");
    0
}

unsafe extern "C" fn fuse_client_truncate(arg1: *const c_char, arg2: sys::off_t) -> c_int {
    println!("unimplemented truncate");
    0
}

unsafe extern "C" fn fuse_client_utimens(arg1: *const c_char, tv: *const sys::timespec) -> c_int {
    println!("unimplemented utimens");
    0
}

unsafe extern "C" fn fuse_client_write(
    path: *const c_char,
    buf: *const c_char,
    size: usize,
    offest: sys::off_t,
    info: *mut sys::fuse_file_info,
) -> c_int {
    // println!("unimplemented write");

    if (*info).fh == 0 {
        (*info).fh = sys::open(path, sys::O_WRONLY as i32).try_into().unwrap();
    }

    // FIXME: error handle on invalid file handle

    libc::write((*info).fh.try_into().unwrap(), buf as *mut c_void, size);

    // FIXME: handle offest

    // FIXME: handle write error
    0
}

unsafe extern "C" fn fuse_client_flush(
    path: *const c_char,
    info: *mut sys::fuse_file_info,
) -> c_int {
    0
}

const fn generate_fuse_ops() -> sys::fuse_operations {
    unsafe {
        let mut ops: sys::fuse_operations = MaybeUninit::zeroed().assume_init();
        ops.getattr = Some(fuse_client_getattr);
        ops.readdir = Some(fuse_client_readdir);
        ops.open = Some(fuse_client_open);
        ops.create = Some(fuse_client_create);
        ops.chmod = Some(fuse_client_chmod);
        ops.chown = Some(fuse_client_chown);
        ops.truncate = Some(fuse_client_truncate);
        ops.utimens = Some(fuse_client_utimens);
        ops.write = Some(fuse_client_write);
        ops.flush = Some(fuse_client_flush);
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
