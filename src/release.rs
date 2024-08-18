use chrono::NaiveDate;
use crate::channel::Channel;

pub struct Release {
    /// The release date. (e.g. November 15, 2022)
    pub date: NaiveDate,

    /// Major version codename. (e.g. Ladybug).
    pub codename: String,

    /// Version number displayed in titles. (e.g. 2024.2.1 Canary 7)
    pub version_title: String,

    /// Release channel. (e.g. Canary)
    pub channel: Channel,

    /// Iterator of releases per major version in channel. (e.g. `7` for `Canary 7`)
    pub channel_version: u8,

    /// Release semantic version. (e.g. 2024.2.1.3)
    pub version_number: String,

    /// Build version. (e.g. 242.20224.300.2421.12232258)
    pub build_version: String,
}