extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn testmac(_item: TokenStream) -> TokenStream {
    #[cfg(flag)]
    return r#"println!("Flag passed to proc macro");"#.parse().unwrap();

    #[cfg(not(flag))]
    return r#"println!("Flag not passed to proc macro");"#.parse().unwrap();
}
