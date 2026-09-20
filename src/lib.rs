#![doc = include_str!("../docs/Home.md")]
#![doc = ""]
#![cfg_attr(feature = "dioxus", doc = "[dioxus-guide]: crate::dioxus")]
#![cfg_attr(
    not(feature = "dioxus"),
    doc = "[dioxus-guide]: https://docs.rs/ferriswatch/latest/ferriswatch/dioxus/index.html"
)]
#![cfg_attr(
    feature = "dioxus-components",
    doc = "[components-guide]: crate::components"
)]
#![cfg_attr(
    not(feature = "dioxus-components"),
    doc = "[components-guide]: https://docs.rs/ferriswatch/latest/ferriswatch/components/index.html"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(auto_cfg))]

#[cfg(feature = "dioxus-components")]
pub use ferriswatch_components::*;
pub use ferriswatch_core::*;
