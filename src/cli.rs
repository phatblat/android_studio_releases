//!
//! cli.rs
//!

use clap::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Parses command line arguments.
pub(crate) fn cli() -> Command {
    Command::new("android_studio_releases")
        .about("CLI for Android Studio app updates")
        .version(VERSION)
        .author("Ben Chatelain")
        .after_help(
            "This tool parses the content of the Android Studio Releases List page: \
            https://plugins.jetbrains.com/docs/intellij/android-studio-releases-list.html",
        )
}

/* ---------------------------------------------------------------------------------------------- */

#[test]
fn verify_cli() {
    cli().debug_assert();
}
