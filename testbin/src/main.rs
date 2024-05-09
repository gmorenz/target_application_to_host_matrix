#![allow(unexpected_cfgs)]

fn main() {
    #[cfg(flag)]
    println!("Flag passed to bin");

    #[cfg(not(flag))]
    println!("Flag not passed to bin");

    testmac::testmac!();

    println!(include_str!("../target/build.out"));
}
