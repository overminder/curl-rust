extern crate libc;
extern crate url;

#[macro_use]
extern crate log;

extern crate curl_sys as curl_ffi;

#[cfg(all(unix, not(target_os = "macos")))]
extern crate openssl_sys as openssl;

#[cfg(unix)]
extern crate openssl as openssl_rust;

pub use ffi::easy::ProgressCb;
pub use ffi::err::ErrCode;

// Version accessors
pub use ffi::version::{
    Version,
    version,
    version_info,
    Protocols
};

pub mod ffi;
pub mod http;
