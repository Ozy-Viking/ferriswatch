//! Errors exposed by the Ferriswatch public API.

#[doc(inline)]
pub use crate::catalogue::ResolveError;
#[doc(inline)]
pub use crate::color::{ChannelError, ColorError, RangeErrorReason, WrappingRangeErrorReason};
#[doc(inline)]
pub use crate::palette::ParseAccentError;
#[doc(inline)]
pub use crate::theme::ThemeError;
#[doc(inline)]
pub use crate::theme_variant::IdentityError;
