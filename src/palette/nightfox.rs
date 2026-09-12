//! Nightfox palette variants and typed factories.

pub mod carbonfox;
pub mod dawnfox;
pub mod dayfox;
pub mod duskfox;
// Upstream names both the family and its original variant Nightfox.
#[allow(clippy::module_inception)]
pub mod nightfox;
pub mod nordfox;
pub mod terafox;

pub use carbonfox::Carbonfox;
pub use dawnfox::Dawnfox;
pub use dayfox::Dayfox;
pub use duskfox::Duskfox;
pub use nightfox::Nightfox;
pub use nordfox::Nordfox;
pub use terafox::Terafox;
