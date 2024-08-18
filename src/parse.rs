//!
//! parse.rs
//!

use std::num::ParseIntError;
use crate::release::Release;
use crate::{GenericResult, SELECTORS};
use chrono::NaiveDate;
use lazy_static::lazy_static;
use scraper::{ElementRef, Html, Selector};
use regex::Regex;
use crate::channel::Channel;

/// Finds articles in the HTML.
///
/// # Arguments
///
/// - `content` - The HTML to parse.
///
/// # Returns
///
/// A result containing a vector of releases.
pub fn parse_releases(content: String) -> GenericResult<Vec<Release>> {
    let document = Html::parse_document(&content);
    let mut releases: Vec<Release> = Vec::new();

    // Each year is in a separate table. For now, only parse the first table for 2024 releases.
    for container in document.select(&SELECTORS.releases_2024) {
        // Contains codename, semver (without 4th), channel and channel version
        // e.g. Ladybug | 2024.2.1 Canary 7
        let release_name = parse_release_name(&container, &SELECTORS.release_name).expect("release_name");
        let codename = parse_codename(&release_name)?;

        let version_title = parse_version_title(&release_name)?;
        let channel = parse_channel(&version_title)?;
        let channel_version = parse_channel_version(&version_title)?;

        let date = parse_date(&container, &SELECTORS.date).expect("date");
        let version_number = parse_semver(&container, &SELECTORS.semver).expect("semver");
        let build_version = parse_build_version(&container, &SELECTORS.build_version).expect("build_version");

        let release = Release {
            date,
            codename,
            version_title,
            channel,
            channel_version,
            version_number,
            build_version,
        };

        releases.push(release);
    }

    Ok(releases)
}

/// Parses the release name.
///
/// # Arguments
///
/// - `element` - The HTML ElementRef to parse.
/// - `selector` - The selector to use.
pub fn parse_release_name(element: &ElementRef, selector: &Selector) -> GenericResult<String> {
    Ok(element.select(selector).next().ok_or("No release row found")?.inner_html())
}

/// Parses the codename.
///
/// # Arguments
///
/// - `release_name` - The release name text.
pub fn parse_codename(release_name: &String) -> GenericResult<String> {
    let re = Regex::new(r"^(.+\w)\s*\|").unwrap(); // Match everything up to the pipe, allowing optional whitespace

    let text_before_pipe = re.captures(release_name.as_str())
        .and_then(|cap| cap.get(1))
        .map_or("", |m| m.as_str());

    println!("Text before pipe: {}", text_before_pipe);

    Ok(text_before_pipe.to_string())
}

/// Parses the version title.
///
/// # Arguments
///
/// - `release_name` - The release name text.
pub fn parse_version_title(release_name: &String) -> GenericResult<String> {
    let re = Regex::new(r"^.*\|\s*(\w.+)").unwrap(); // Match everything after the pipe and some optional whitespace

    let text_after_pipe = re.captures(release_name.as_str())
        .and_then(|cap| cap.get(1))
        .map_or("", |m| m.as_str());

    println!("Text after pipe: {}", text_after_pipe);

    Ok(text_after_pipe.to_string())
}

/// Parses the channel name out of the version title.
///
/// # Arguments
///
/// - `version_title` - The version title text.
pub fn parse_channel(version_title: &String) -> Result<Channel, String> {
    // e.g. 2024.2.1 Canary 7
    // 2nd group is the channel name
    let re = Regex::new(r"^(\d+\.\d+\.\d+)\s?(\w+)\s?(\d)?").unwrap();

    let channel_name = re.captures(version_title.as_str())
        .and_then(|cap| cap.get(2))
        .map_or("", |m| m.as_str());

    println!("Channel name: {}", channel_name);

    Channel::try_from(channel_name.to_string())
}

/// Parses the channel version out of the version title.
///
/// # Arguments
///
/// - `version_title` - The version title text.
pub fn parse_channel_version(version_title: &String) -> Result<u8, ParseIntError> {
    // e.g. 2024.2.1 Canary 7
    // 3rd group is the channel version
    let re = Regex::new(r"^(\d+\.\d+\.\d+)\s?(\w+)\s?(\d)?").unwrap();

    let channel_version = re.captures(version_title.as_str())
        .and_then(|cap| cap.get(3))
        .map_or("", |m| m.as_str());

    println!("Channel version: {}", channel_version);

    channel_version.parse()
}

/// Parses the article date.
///
/// # Arguments
///
/// - `element` - The HTML ElementRef to parse.
/// - `selector` - The selector to use.
pub fn parse_date(element: &ElementRef, selector: &Selector) -> GenericResult<NaiveDate> {
    let date_string = element
        .select(selector)
        .next()
        .ok_or("No date found")?
        .inner_html();

    Ok(NaiveDate::parse_from_str(
        date_string.as_str(),
        "%B %d, %Y",
    )?)
}

/// Parses the semantic version.
///
/// # Arguments
///
/// - `element` - The HTML ElementRef to parse.
/// - `selector` - The selector to use.
pub fn parse_semver(element: &ElementRef, selector: &Selector) -> GenericResult<String> {
    Ok(element.select(selector)
        .next()
        .ok_or("No semantic version found")?
        .inner_html())
}

/// Parses the build version.
///
/// # Arguments
///
/// - `element` - The HTML ElementRef to parse.
/// - `selector` - The selector to use.
pub fn parse_build_version(element: &ElementRef, selector: &Selector) -> GenericResult<String> {
    Ok(element.select(selector)
        .next()
        .ok_or("No build version found")?
        .inner_html())
}

/* ---------------------------------------------------------------------------------------------- */

lazy_static! {
    static ref HTML: String = r###"
<section class="chapter h1-related">
  <h2 id="2024" data-toc="2024" class="article__h2">
    <span class="title"
      ><span class="title__content">2024.*</span>﻿<a
        data-test="internal-link permalink"
        rel=""
        class="link-nude permalink"
        href="/docs/intellij/android-studio-releases-list.html#2024"
        ><span
          data-clipboard-text="https://plugins.jetbrains.com/docs/intellij/android-studio-releases-list.html#2024"
          ><svg
            viewBox="0 0 24 24"
            class="wt-icon wt-icon_size_s permalink__icon permalink__icon--size-s"
          >
            <path
              d="M21.207 4.793a4.536 4.536 0 0 0-6.414 0l-4.5 4.5 1.414 1.414 4.5-4.5a2.536 2.536 0 0 1 3.586 3.586l-4.5 4.5 1.414 1.414 4.5-4.5a4.536 4.536 0 0 0 0-6.414z"
            ></path>
            <path
              d="M8.328 16.258a2.536 2.536 0 1 1-3.586-3.586l4.5-4.5-1.414-1.414-4.5 4.5a4.535 4.535 0 0 0 6.414 6.414l4.5-4.5-1.414-1.414z"
            ></path></svg></span></a
    ></span>
  </h2>
  <div class="table h2-related">
    <div
      class="table__wrapper table__wrapper--wide table__wrapper--without-scroll"
    >
      <table class="table__content table__content--wide" id="slaxfo_21">
        <thead class="table__thead">
          <tr class="table__tr" id="slaxfo_22">
            <th class="table__th" id="slaxfo_54">
              <p class="article__p">Release Name</p>
            </th>
            <th class="table__th" id="slaxfo_55">
              <p class="article__p">Channel</p>
            </th>
            <th class="table__th" id="slaxfo_56">
              <p class="article__p">Release Date</p>
            </th>
            <th class="table__th" id="slaxfo_57">
              <p class="article__p">Version</p>
            </th>
            <th class="table__th" id="slaxfo_58">
              <p class="article__p">IntelliJ IDEA Version</p>
            </th>
          </tr>
        </thead>
        <tbody class="table__tbody">
          <tr class="table__tr" id="slaxfo_23">
            <td class="table__td" id="slaxfo_59">
              <p class="article__p child">Ladybug | 2024.2.1 Canary 7</p>
            </td>
            <td class="table__td" id="slaxfo_60">
              <figure id="slaxfo_64" class="article__figure child">
                <img
                  src="https://img.shields.io/badge/-Canary-lightgrey?style=flat-square"
                  class="article__img"
                  alt="Canary"
                  title="Canary"
                />
              </figure>
            </td>
            <td class="table__td" id="slaxfo_61">
              <p class="article__p child">August 15, 2024</p>
            </td>
            <td class="table__td" id="slaxfo_62">
              <p class="article__p child">
                <span class="control" id="slaxfo_65">2024.2.1.3</span>
              </p>
              <p id="slaxfo_66" class="article__p child">
                AI-242.20224.300.2421.12232258
              </p>
            </td>
            <td class="table__td" id="slaxfo_63">
              <p class="article__p child">
                <span class="control" id="slaxfo_67">2024.2</span>
              </p>
              <p id="slaxfo_68" class="article__p child">242.20224.300</p>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</section>
    "###.to_string();
}

#[test]
fn test_parse_release_name() {
    let fragment = Html::parse_fragment(&HTML);

    // test parsing using local selector
    let selector = Selector::parse(r#"table.table__content tbody tr td p"#).unwrap();
    let element = fragment.select(&selector).next().unwrap();
    println!("{}", element.inner_html());

    let title = parse_release_name(&fragment.root_element(), &SELECTORS.release_name).unwrap();

    assert_eq!(title, "Ladybug | 2024.2.1 Canary 7");
}

#[test]
fn test_parse_codename() {
    let input_string = "Ladybug | 2024.2.1 Canary 7".to_string();
    let codename = parse_codename(&input_string).unwrap();

    assert_eq!(codename, "Ladybug");
}

#[test]
fn test_parse_codename_with_space() {
    let input_string = "Koala Feature Drop | 2024.1.2 RC 1".to_string();
    let codename = parse_codename(&input_string).unwrap();

    assert_eq!(codename, "Koala Feature Drop");
}

#[test]
fn test_parse_version_title() {
    let input_string = "Koala Feature Drop | 2024.1.2 RC 1".to_string();
    let version_title = parse_version_title(&input_string).unwrap();

    assert_eq!(version_title, "2024.1.2 RC 1");
}

#[test]
fn test_parse_channel() {
    let input_string = "2024.2.1 Canary 7".to_string();
    let channel = parse_channel(&input_string).unwrap();

    assert_eq!(channel, Channel::Canary);
}

#[test]
fn test_parse_channel_version() {
    let input_string = "2024.2.1 Canary 7".to_string();
    let channel = parse_channel_version(&input_string).unwrap();

    assert_eq!(channel, 7);
}

#[test]
fn test_parse_date() {
    let fragment = Html::parse_fragment(&HTML);

    // test parsing using local selector
    let selector = Selector::parse(r#"table.table__content tbody tr td:nth-child(3) p"#).unwrap();
    let element = fragment.select(&selector).next().unwrap();
    println!("{}", element.inner_html());

    let date = parse_date(&fragment.root_element(), &SELECTORS.date).unwrap();
    let expected_date = NaiveDate::parse_from_str("August 15, 2024", "%B %d, %Y").unwrap();

    assert_eq!(date, expected_date);
}

#[test]
fn test_parse_releases() {

    let articles = parse_releases((&HTML).to_string())
        .expect("Err collecting releases");

    assert_eq!(articles.len(), 1);
}

