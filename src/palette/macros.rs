// Shared construction; each concrete palette declares metadata and semantic mappings.
macro_rules! define_palette {
    (
        $palette:ident, $label:literal,
        identity($id:literal, $family:literal, $family_name:literal, $variant:literal, $variant_name:literal, $appearance:ident, $contrast:expr);
        support($support:ident);
        sources: $sources:expr;
        colors { $($color:ident = $hex:expr),+ $(,)? }
        accents { $($accent:ident = $accent_color:ident => ($accent_id:literal, $accent_name:literal)),+ $(,)? }
        default $default:ident => $default_id:literal;
        roles($primary:ident) { $($role:ident : $value:expr),+ $(,)? }
    ) => {
        #[doc = concat!($label, " raw palette and semantic theme factory.")]
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct $palette;
        impl $palette {
            $(pub const $color: $crate::color::Color = $hex;)+
            /// Metadata, supported accents and source provenance for this palette.
            pub const REGISTRATION: $crate::catalogue::PaletteRegistration = $crate::catalogue::PaletteRegistration {
                metadata: $crate::theme_variant::ThemeMetadata {
                    id: std::borrow::Cow::Borrowed($id), family_id: std::borrow::Cow::Borrowed($family),
                    family_name: std::borrow::Cow::Borrowed($family_name), variant_id: std::borrow::Cow::Borrowed($variant),
                    variant_name: std::borrow::Cow::Borrowed($variant_name), name: std::borrow::Cow::Borrowed($label),
                    appearance: $crate::theme_variant::Appearance::$appearance, contrast: $contrast,
                    support: $crate::theme_variant::ThemeSupport::$support,
                },
                accents: &[$($crate::catalogue::AccentRegistration {
                    id: $accent_id, name: $accent_name, color: Self::$accent_color,
                    factory: <Self as $crate::theme_variant::ThemePalette>::variant::<$accent>,
                }),+],
                default_accent: $default_id, sources: $sources,
                raw_colors: &[$((stringify!($color), Self::$color)),+],
                factory: <Self as $crate::theme_variant::ThemePalette>::variant::<$crate::palette::NoAccent>,
            };
        }
        impl $crate::palette::Palette for $palette {
            fn registration() -> &'static $crate::catalogue::PaletteRegistration {
                &Self::REGISTRATION
            }
        }
        $(
            #[doc = concat!($accent_name, " accent for ", $label, ".")]
            #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
            pub struct $accent;
            impl $crate::palette::Accent<$palette> for $accent {
                const ACCENT: Option<$crate::color::Color> = Some($palette::$accent_color);
                const ID: Option<&'static str> = Some($accent_id);
                const NAME: Option<&'static str> = Some($accent_name);
            }
        )+
        impl $crate::theme_variant::ThemePalette for $palette {
            fn variant<A: $crate::palette::Accent<Self>>() -> $crate::theme_variant::ThemeVariant {
                let $primary = A::ACCENT.unwrap_or(Self::$default);
                $crate::theme_variant::ThemeVariant::from_palette(
                    &Self::REGISTRATION.metadata,
                    $crate::theme_variant::ThemeVariantColors { $($role: $value,)+ },
                    $crate::palette::selected_accent::<Self, A>(),
                )
            }
        }
    };
}
