//! Emoji constants used by `wasm-bingen-build`.
//!
//! For the woefully unfamiliar:
//!
//! > Emoji are ideograms and smileys used in electronic messages and web
//! > pages. Emoji exist in various genres, including facial expressions, common
//! > objects, places and types of weather, and animals. They are much like
//! > emoticons, but emoji are actual pictures instead of typographics.
//!
//! -- https://en.wikipedia.org/wiki/Emoji

#![allow(missing_docs)]

use console::Emoji;

pub static TARGET: Emoji = Emoji("🎯  ", "");
pub static CYCLONE: Emoji = Emoji("🌀  ", "");
pub static DOWN_ARROW: Emoji = Emoji("⬇️  ", "");
pub static SPARKLE: Emoji = Emoji("✨  ", ":-)");
pub static CONFIG: Emoji = Emoji("⚙️. ", "");
pub static PACKAGE: Emoji = Emoji("📦  ", ":-)");
pub static WARN: Emoji = Emoji("⚠️  ", ":-)");
