// Shared construction for built-in palettes. Semantic mappings remain in each variant.
macro_rules! define_palette {
    (
        $palette:ident, $label:literal,
        colors { $($color:ident = $hex:literal),+ $(,)? }
        accents { $($accent:ident = $accent_color:ident),+ $(,)? }
        default $default:ident;
        roles($primary:ident) { $($role:ident : $value:expr),+ $(,)? }
    ) => {
        #[doc = concat!($label, " raw palette and semantic theme factory.")]
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct $palette;
        impl $palette {
            $(pub const $color: $crate::color::Color = $crate::color::Color::hex($hex);)+
        }
        impl $crate::palette::Palette for $palette {}
        $(
            #[doc = concat!(stringify!($accent), " accent for ", $label, ".")]
            #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
            pub struct $accent;
            impl $crate::palette::Accent<$palette> for $accent {
                const ACCENT: Option<$crate::color::Color> = Some($palette::$accent_color);
                const NAME: Option<&'static str> = Some(stringify!($accent));
            }
        )+
        impl $crate::theme_variant::ThemePalette for $palette {
            fn variant<A: $crate::palette::Accent<Self>>() -> $crate::theme_variant::ThemeVariant {
                let $primary = A::ACCENT.unwrap_or(Self::$default);
                $crate::theme_variant::ThemeVariant::new(
                    $label,
                    $crate::theme_variant::ThemeVariantColors {
                        $($role: $value,)+
                    },
                    A::ACCENT,
                    A::NAME.map(str::to_owned),
                )
            }
        }
    };
}
