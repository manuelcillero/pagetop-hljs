//! Configuration settings for **HighlightJS** module.
//!
//! Example:
//!
//! ```toml
//! [hljs]
//! theme = "zenburn"
//! library = "common"
//! tabsize = 8
//! ```
//!
//! Use:
//!
//! ```rust
//! use pagetop_hljs::config;
//!
//! assert_eq!(config::SETTINGS.hljs.theme, "zenburn");
//! ```
//! See [`pagetop::config`](https://docs.rs/pagetop/latest/pagetop/config/) to learn how **PageTop**
//! read configuration files and use settings.

use pagetop::prelude::*;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Configuration settings for the [`[hljs]`](Hljs) section (see [`SETTINGS`]).
pub struct Settings {
    pub hljs: Hljs,
}
#[derive(Debug, Deserialize)]
/// [`Settings`] section `[hljs]`.
pub struct Hljs {
    /// Default theme to display code snippets on web pages, written in *kebab-case* (see
    /// [`HljsTheme`](crate::HljsTheme)).
    /// Default value: *"default"*
    pub theme: String,
    /// You can utilize the ***core*** library to import the language specific to each code snippet
    /// enabled in the given context (see [`enable()`](crate::HighlightJS::enable())). Furthermore,
    /// the ***common*** library provides support for approximately 40 popular languages. However,
    /// if you exclusively rely on the common library, you will only have access to these preloaded
    /// languages.
    /// Default value: *"core"*
    pub library: String,
    /// Number of spaces for *tab* character.
    /// Default value: *4*
    pub tabsize: u8,
}

use_config!(SETTINGS as Settings,
    // [hljs]
    "hljs.theme"   => "default",
    "hljs.library" => "core",
    "hljs.tabsize" => 4,
);
