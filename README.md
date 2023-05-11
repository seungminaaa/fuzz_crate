# Installation
You may need to install the cargo-fuzz library to get the cargo fuzz subcommand. Use
    $ cargo install cargo-fuzz
cargo-fuzz is documented in the Rust Fuzz Book.

# fuzz_crate
Those folders are from crates.io.  Those are used for fuzzing in cargo afl. All inputs are from std::Read


each folder name is crate name. 

# how to build
$cargo afl build

# how to fuzz
#cargo afl fuzz -i [input] -o out target/debug/[binary file name] 
