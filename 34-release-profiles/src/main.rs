fn main() {
    // In Rust, release profiles are predefined, customizable profiles with different configurations
    // that allow a programmer to have more control over various options for compiling code.
    // Each profile is configured independently of the others.
    //
    // Cargo has two main profiles: the dev profile Cargo uses when you run cargo build, and the release profile Cargo uses when you run cargo build --release.
    // The dev profile is defined with good defaults for development, and
    // the release profile has good defaults for release builds.

    "cargo build";
    "cargo build --release";

    // Cargo has default settings for each of the profiles
    // that apply when you haven’t explicitly added any [profile.*] sections in the project’s Cargo.toml file
}
