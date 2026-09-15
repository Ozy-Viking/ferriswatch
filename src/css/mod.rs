//! Framework-independent semantic CSS. See the [CSS guide](crate::css).
#![doc = include_str!("../../docs/Css.md")]

mod variables;

pub use variables::theme_css;

/// Optional, palette-independent color classes. No styles are loaded implicitly.

pub const DEFAULT_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/default.css"));

/// Optional default color stylesheet asset for Dioxus's document integration.
/// Use `document::Stylesheet { href: DEFAULT_STYLESHEET }` in an asset-aware `dx` build.
#[cfg(feature = "dioxus")]

pub const DEFAULT_STYLESHEET: dioxus::prelude::Asset = {

    use dioxus::prelude::*;

    asset!("/src/css/default.css")
};

/// Where a theme provider publishes its CSS variables and color scheme.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]

pub enum ThemeScope {
    /// The provider's DOM subtree. Nested providers can override inherited roles.
    #[default]
    Scoped,
    /// The current document's root. Use one root provider per document.
    Root,
}

/// Load the default color classes once through Dioxus's document integration.
/// Requires a renderer supporting Dioxus documents and an asset-aware `dx` build.
#[cfg(feature = "dioxus")]
#[dioxus::prelude::component]

pub fn DefaultStyles() -> dioxus::prelude::Element {

    use dioxus::prelude::*;

    rsx! { document::Stylesheet { href: DEFAULT_STYLESHEET } }
}
