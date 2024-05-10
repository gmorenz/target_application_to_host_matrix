#![feature(linkage)]
#![allow(unexpected_cfgs)]

use std::fs::File;
use std::io::Write;

extern {
    #[linkage = "extern_weak"]
    static FLAG_LINKER: *const std::ffi::c_void;
}

fn main() {
    let mut f = File::create("target/build.out").unwrap();

    #[cfg(flag)]
    writeln!(f, "Flag passed to build script").unwrap();

    #[cfg(not(flag))]
    writeln!(f, "Flag not passed to build script").unwrap();

    if (unsafe{ FLAG_LINKER } as usize != 0) {
        write!(f, "Linker passed to build script").unwrap();
    } else {
        write!(f, "Linker not passed to build script").unwrap();
    }
}