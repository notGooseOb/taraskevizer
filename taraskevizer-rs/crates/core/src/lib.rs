//! `taraskevizer` — Belarusian orthography converter (Narkamauka → Taraskevica).
//!
//! The conversion core behind the [`tarask`](https://crates.io/crates/tarask)
//! CLI, the `taraskevizer` npm package (via WebAssembly) and the JSON
//! dictionaries published with every GitHub release.
//!
//! # Pipelines
//!
//! * [`tarask`] — full taraskevization (orthography + alphabet).
//! * [`alphabetic`] — alphabet-only conversion, no taraskevization.
//! * [`phonetic`] — phonetic conversion (experimental).
//!
//! # Example
//!
//! ```rust
//! use taraskevizer::{tarask, TaraskConfig};
//!
//! let cfg = TaraskConfig::default();
//! assert_eq!(tarask("планета", &cfg), "плянэта");
//! ```
//!
//! # Configuration
//!
//! [`TaraskConfig`] selects the alphabet, the `і→й` mode, variation output,
//! highlight [`wrappers`] and a few text-level replacements.
//! [`html_config_options`] mirrors the web preset (`<tarF>`/`<tarL>`/`<tarH>`
//! wrappers, `<br>` newlines).
//!
//! For parallel conversion of large inputs, split the text with
//! [`text::split_into_chunks`] and convert each chunk separately — chunked
//! conversion concatenates to the same result as a single call.

pub mod config;
pub mod dict;
pub mod pipeline;
pub mod text;
pub mod wrappers;

pub use config::TaraskConfig;
pub use pipeline::{apply_highlight_diff, run_alphabetic, run_phonetic, run_tarask};
pub use wrappers::{html_config_options, ANSI_COLOR_WRAPPERS, HTML_WRAPPERS};

/// Run the full taraskevization pipeline.
pub fn tarask(text: &str, cfg: &TaraskConfig) -> String {
    run_tarask(text, cfg)
}

/// Run alphabet-only pipeline.
pub fn alphabetic(text: &str, cfg: &TaraskConfig) -> String {
    run_alphabetic(text, cfg)
}

/// Run phonetic pipeline.
pub fn phonetic(text: &str, cfg: &TaraskConfig) -> String {
    run_phonetic(text, cfg)
}
