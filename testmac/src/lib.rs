#![allow(unexpected_cfgs)]

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn testmac(_item: TokenStream) -> TokenStream {
    #[cfg(flag)]
    let our_str = "Flag passed to proc macro";

    #[cfg(not(flag))]
    let our_str = "Flag not passed to proc macro";

    let proc_macro = format!("{} from proc macro", shared_dep::output());

    format!("println!(\"{}\n{}\");", our_str, proc_macro).parse().unwrap()
}
