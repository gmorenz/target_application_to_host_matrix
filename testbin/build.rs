use std::fs::File;
use std::io::Write;

fn main() {
    let mut f = File::create("target/build.out").unwrap();

    #[cfg(flag)]
    writeln!(f, "Flag passed to build script").unwrap();

    #[cfg(not(flag))]
    writeln!(f, "Flag not passed to build script").unwrap();
}