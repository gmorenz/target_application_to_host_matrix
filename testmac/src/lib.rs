#![allow(unexpected_cfgs)]
#![feature(linkage)]

extern {
    #[linkage = "extern_weak"]
    static FLAG_LINKER: *const std::ffi::c_void;
}

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn testmac(_item: TokenStream) -> TokenStream {
    #[cfg(flag)]
    let our_str = "Flag passed to proc macro";

    #[cfg(not(flag))]
    let our_str = "Flag not passed to proc macro";

    let proc_macro = format!("{} from proc macro", shared_dep::output());

    let linker_str =
    if (unsafe{ FLAG_LINKER } as usize != 0) {
        "Linker passed to proc macro"
    } else {
        "Linker not passed to proc macro"
    };

    format!("println!(\"{}\n{}\n{}\");", our_str, proc_macro, linker_str).parse().unwrap()
}
