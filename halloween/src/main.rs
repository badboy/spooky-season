use std::ffi::CString;

use spooky_season::spooky;

fn main() {
    let spook = "👻🎃👻 SPOOKY! 👻🎃👻".to_string().into_bytes();
    let spook = CString::new(spook).unwrap();
    spooky! {
        libc::puts(spook.as_ptr());
    }
}
