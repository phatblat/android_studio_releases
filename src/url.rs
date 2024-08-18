//!
//! url.rs
//!

use url::Url;

use crate::{GenericResult, ANDROID_STUDIO_RELEASES_LIST, APP_USER_AGENT};

/// Gets a URL and returns the body of the response.
///
/// # Arguments
///
/// - `url` - The URL to get.
pub fn get(url: String) -> GenericResult<String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let res = client.get(url).send()?;
    let body = res.text()?;
    Ok(body)
}

/// Builds the release list URL.
pub(crate) fn build_releases_url() -> Url {
    Url::parse(ANDROID_STUDIO_RELEASES_LIST).unwrap()
}

/* ---------------------------------------------------------------------------------------------- */

#[test]
fn test_get() {
    let body = get(ANDROID_STUDIO_RELEASES_LIST.to_string()).unwrap();
    assert!(!body.is_empty());
}

#[test]
fn test_build_notes_url() {
    let expected_url = Url::parse(ANDROID_STUDIO_RELEASES_LIST).unwrap();
    let url = build_releases_url();
    assert_eq!(url, expected_url);
}
