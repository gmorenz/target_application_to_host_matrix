#![feature(linkage)]
#![allow(unexpected_cfgs)]

extern {
    #[linkage = "extern_weak"]
    static FLAG_LINKER: *const std::ffi::c_void;
}

fn main() {
    #[cfg(flag)]
    println!("Flag passed to bin");

    #[cfg(not(flag))]
    println!("Flag not passed to bin");

    println!("{} from bin", shared_dep::output());

    // Incidentally I dispute that this is entirely safe,
    // we're just looking up the value of a symbol. And the
    // compiler knows it.
    if (unsafe{ FLAG_LINKER } as usize != 0) {
        println!("Linker passed to bin");
    } else {
        println!("Linker not passed to bin");
    }

    println!("");

    testmac::testmac!();

    println!("");

    println!(include_str!("../target/build.out"));

}
