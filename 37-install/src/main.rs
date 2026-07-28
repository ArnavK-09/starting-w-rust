// Installing Binaries with cargo install
//
//
// A binary target is the runnable program that is created if the crate has a src/main.rs file or another file specified as a binary,
// as opposed to a library target that isn’t runnable on its own but is suitable for including within other programs.

// All binaries installed with cargo install are stored in the installation root’s bin folder.
// If you installed Rust using rustup.rs and don’t have any custom configurations, this directory will be $HOME/.cargo/bin.

// The cargo install command allows you to install and use binary crates locally

// cargo install rigrep
//
//
// The second-to-last line of the output shows the location and the name of the installed binary,
// which in the case of ripgrep is rg. As long as the installation directory is in your $PATH, as mentioned previously,
// you can then run rg --help and start using a faster

// Cargo is designed so that you can extend it with new subcommands without having to modify it.
//
//  If a binary in your $PATH is named cargo-something, you can run it as if it were a Cargo subcommand by running cargo something

// Custom commands like this are also listed when you run `cargo --list`

fn main() {}
