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

macro_rules! catppuccin_variant {
    ($palette:ident, $label:literal) => {
        /// Rosewater accent for this Catppuccin flavour.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct Rosewater;
        impl Accent<$palette> for Rosewater {
            const ACCENT: Option<Color> = Some($palette::ROSEWATER);
            const NAME: Option<&'static str> = Some("Rosewater");
        }
        /// Flamingo accent for this Catppuccin flavour.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct Flamingo;
        impl Accent<$palette> for Flamingo {
            const ACCENT: Option<Color> = Some($palette::FLAMINGO);
            const NAME: Option<&'static str> = Some("Flamingo");
        }
        /// Pink accent for this Catppuccin flavour.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct Pink;
        impl Accent<$palette> for Pink {
            const ACCENT: Option<Color> = Some($palette::PINK);
            const NAME: Option<&'static str> = Some("Pink");
        }
        /// Mauve accent for this Catppuccin flavour.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct Mauve;
        impl Accent<$palette> for Mauve {
            const ACCENT: Option<Color> = Some($palette::MAUVE);
            const NAME: Option<&'static str> = Some("Mauve");
        }
        /// Red accent for this Catppuccin flavour.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct Red;
        impl Accent<$palette> for Red {
            const ACCENT: Option<Color> = Some($palette::RED);
            const NAME: Option<&'static str> = Some("Red");
        }
        /// Maroon accent for this Catppuccin flavour.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct Maroon;
        impl Accent<$palette> for Maroon {
            const ACCENT: Option<Color> = Some($palette::MAROON);
            const NAME: Option<&'static str> = Some("Maroon");
        }
        /// Peach accent for this Catppuccin flavour.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct Peach;
        impl Accent<$palette> for Peach {
            const ACCENT: Option<Color> = Some($palette::PEACH);
            const NAME: Option<&'static str> = Some("Peach");
        }
        /// Yellow accent for this Catppuccin flavour.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct Yellow;
        impl Accent<$palette> for Yellow {
            const ACCENT: Option<Color> = Some($palette::YELLOW);
            const NAME: Option<&'static str> = Some("Yellow");
        }
        /// Green accent for this Catppuccin flavour.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct Green;
        impl Accent<$palette> for Green {
            const ACCENT: Option<Color> = Some($palette::GREEN);
            const NAME: Option<&'static str> = Some("Green");
        }
        /// Teal accent for this Catppuccin flavour.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct Teal;
        impl Accent<$palette> for Teal {
            const ACCENT: Option<Color> = Some($palette::TEAL);
            const NAME: Option<&'static str> = Some("Teal");
        }
        /// Sky accent for this Catppuccin flavour.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct Sky;
        impl Accent<$palette> for Sky {
            const ACCENT: Option<Color> = Some($palette::SKY);
            const NAME: Option<&'static str> = Some("Sky");
        }
        /// Sapphire accent for this Catppuccin flavour.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct Sapphire;
        impl Accent<$palette> for Sapphire {
            const ACCENT: Option<Color> = Some($palette::SAPPHIRE);
            const NAME: Option<&'static str> = Some("Sapphire");
        }
        /// Blue accent for this Catppuccin flavour.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct Blue;
        impl Accent<$palette> for Blue {
            const ACCENT: Option<Color> = Some($palette::BLUE);
            const NAME: Option<&'static str> = Some("Blue");
        }
        /// Lavender accent for this Catppuccin flavour.
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct Lavender;
        impl Accent<$palette> for Lavender {
            const ACCENT: Option<Color> = Some($palette::LAVENDER);
            const NAME: Option<&'static str> = Some("Lavender");
        }
        impl Palette for $palette {}

        impl ThemePalette for $palette {
            /// Resolves semantic colours with the selected accent, defaulting to mauve.
            /// Hover darkens linear RGB by 15% and preserves alpha.
            fn variant<A>() -> ThemeVariant
            where
                A: Accent<Self>,
            {
                let accent = A::ACCENT.unwrap_or(Self::MAUVE);
                // Darken linear RGB by 15%, preserving opacity, including transparent accents.
                let hover = crate::palette::primary_hover(accent);
                ThemeVariant::new(
                    $label,
                    ThemeVariantColors {
                        surface: crate::theme_variant::SurfaceColors {
                            background: Self::BASE,
                            surface: Self::SURFACE_0,
                            raised: Self::SURFACE_1,
                            overlay: Self::OVERLAY_0,
                            hover: Self::SURFACE_1,
                        },
                        surface_alt: crate::theme_variant::SurfaceColors {
                            background: Self::BASE,
                            surface: Self::SURFACE_0,
                            raised: Self::SURFACE_1,
                            overlay: Self::OVERLAY_0,
                            hover: Self::SURFACE_1,
                        },
                        text: crate::theme_variant::TextColors {
                            normal: Self::TEXT,
                            muted: Self::SUBTEXT_1,
                            subtle: Self::OVERLAY_1,
                            on_primary: crate::palette::action_text(accent),
                            on_secondary: crate::palette::action_text(Self::BLUE),
                        },
                        primary: crate::theme_variant::ActionColors {
                            normal: accent,
                            hover,
                            pressed: accent,
                            muted: Self::SURFACE_2,
                        },
                        secondary: crate::theme_variant::ActionColors {
                            normal: Self::BLUE,
                            hover: crate::palette::primary_hover(Self::BLUE),
                            pressed: Self::BLUE,
                            muted: Self::BLUE,
                        },
                        status: crate::theme_variant::StatusColors {
                            success: Self::GREEN,
                            warning: Self::YELLOW,
                            error: Self::RED,
                            critical: Self::RED,
                            info: Self::TEAL,
                            trace: Self::OVERLAY_1,
                        },
                        border: Self::OVERLAY_0,
                        border_muted: Self::SURFACE_1,
                        focus: accent,
                    },
                    A::ACCENT,
                    A::NAME.map(str::to_owned),
                )
            }
        }
    };
}

pub mod frappe;
pub mod latte;
pub mod macchiato;
pub mod mocha;

pub use frappe::Frappe;
pub use latte::Latte;
pub use macchiato::Macchiato;
pub use mocha::Mocha;
