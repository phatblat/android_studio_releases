use lazy_static::lazy_static;
use crate::cli::cli;
use crate::selectors::Selectors;

mod cli;
mod release;
mod channel;
mod selectors;
mod url;
mod parse;
/* ---------------------------------------------------------------------------------------------- */

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

/// User agent for network requests.
const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Webpage listing new software releases.
const ANDROID_STUDIO_RELEASES_LIST: &str = "https://plugins.jetbrains.com/docs/intellij/android-studio-releases-list.html";

lazy_static! {
    static ref SELECTORS: Selectors = Selectors::new();
}

/* ---------------------------------------------------------------------------------------------- */

/// Executable entry point.
fn main() {
    let args = cli().get_matches();
    let body = url::get(ANDROID_STUDIO_RELEASES_LIST.to_string()).unwrap();
}
