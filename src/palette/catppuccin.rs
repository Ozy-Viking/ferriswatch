//! Catppuccin flavours and their named accents.
//!
//! Colours from <https://github.com/catppuccin/palette>, palette version 1.8.0.
//! All flavours use mauve by default and darken primary hover by 15% in linear RGB.
//!
//! ```
//! use ferriswatch::palette::catppuccin::{latte, frappe, macchiato, mocha};
//! use ferriswatch::theme_variant::ThemePalette;
//!
//! let variants = [
//!     latte::Latte::variant::<latte::Mauve>(),
//!     frappe::Frappe::variant::<frappe::Blue>(),
//!     macchiato::Macchiato::variant::<macchiato::Green>(),
//!     mocha::Mocha::variant::<mocha::Mauve>(),
//! ];
//! assert_eq!(variants[1].name(), "Catppuccin Frappé");
//! assert_eq!(variants[1].accent_name(), Some("Blue"));
//! ```

pub mod frappe;
pub mod latte;
pub mod macchiato;
pub mod mocha;

pub use frappe::Frappe;
pub use latte::Latte;
pub use macchiato::Macchiato;
pub use mocha::Mocha;
