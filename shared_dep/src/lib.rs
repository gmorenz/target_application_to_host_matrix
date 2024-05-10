#![allow(unexpected_cfgs)]

pub fn output() -> &'static str {
    #[cfg(flag)]
    return "Flag passed to shared dep";

    #[cfg(not(flag))]
    return "Flag not passed to shared dep";
}