//!
//! selectors.rs
//!

use scraper::Selector;

/// Collection of scraper Selectors - https://docs.rs/scraper/latest/scraper/
pub(crate) struct Selectors {
    /// Parses the 2024 release table, the first top-level containing values of interest.
    pub(crate) releases_2024: Selector,

    /// Parses the release name.
    pub(crate) release_name: Selector,

    /// Parses the article date.
    pub(crate) date: Selector,

    /// Parses the semantic and build versions.
    pub(crate) versions: Selector,
}

impl Selectors {
    pub(crate) fn new() -> Self {
        Self {
            // WARN: slaxfo_* IDs are likely re-numbered on site regen
            releases_2024: Selector::parse(r#"table#slaxfo_21.table__content"#).unwrap(),
            release_name: Selector::parse(r#"table.table__content tbody tr td p"#).unwrap(),
            // 3rd column
            date: Selector::parse(r#"table.table__content tbody tr td:nth-child(3) p"#).unwrap(),
            // 4th column
            versions: Selector::parse(r#"table.table__content tbody tr td:nth-child(4) p"#).unwrap(),
        }
    }
}
