//! Syntect highlighting themes from Ferriswatch palettes.
//!
//! Enable the `syntect` feature to use this module.

use crate::{
    color::{Color, Rgb, Rgba},
    theme::ThemeVariant,
};

/// Conversion into a Syntect highlighting [`Theme`](syntect::highlighting::Theme).
///
/// Implemented for [`ThemeVariant`].

pub trait SyntectTheme {
    /// Returns this theme as a Syntect highlighting theme.
    #[must_use]

    fn syntect(&self) -> syntect::highlighting::Theme;
}

#[cfg(feature = "syntect")]

impl SyntectTheme for ThemeVariant {
    /// Returns this theme variant as a Syntect highlighting theme.

    fn syntect(&self) -> syntect::highlighting::Theme {

        use syntect::highlighting::{FontStyle, StyleModifier, Theme, ThemeItem, ThemeSettings};

        let colors = self.colors();

        let syntax = &colors.syntax;

        fn item(scope: &str, color: syntect::highlighting::Color) -> ThemeItem {

            ThemeItem {
                scope: scope
                    .parse()
                    .expect("Ferriswatch contains a valid Syntect scope"),
                style: StyleModifier {
                    foreground: Some(color),
                    ..Default::default()
                },
            }
        }

        fn styled_item(
            scope: &str,
            color: syntect::highlighting::Color,
            font_style: FontStyle,
        ) -> ThemeItem {

            ThemeItem {
                scope: scope
                    .parse()
                    .expect("Ferriswatch contains a valid Syntect scope"),
                style: StyleModifier {
                    foreground: Some(color),
                    font_style: Some(font_style),
                    ..Default::default()
                },
            }
        }

        Theme {
            name: Some(self.name().to_owned()),
            author: Some("Ferriswatch".into()),

            settings: ThemeSettings {
                foreground: Some(syntax.foreground.into()),
                ..Default::default()
            },

            scopes: vec![
                // Comments
                item("comment", syntax.comment.into()),
                item(
                    "comment.block.documentation, comment.line.documentation",
                    syntax.documentation.into(),
                ),
                // Constants / literals
                item("constant", syntax.constant.into()),
                item("constant.numeric", syntax.number.into()),
                item("constant.language.boolean", syntax.boolean.into()),
                item("constant.character.escape", syntax.escape.into()),
                // Strings
                item("string", syntax.string.into()),
                // Keywords
                item("keyword", syntax.keyword.into()),
                item("keyword.control", syntax.control_keyword.into()),
                item("keyword.operator", syntax.operator.into()),
                // Storage
                item("storage.type", syntax.type_keyword.into()),
                item("storage.modifier", syntax.modifier.into()),
                // Variables
                item("variable", syntax.variable.into()),
                item("variable.parameter", syntax.parameter.into()),
                item("variable.other.member", syntax.property.into()),
                item("variable.function", syntax.function.into()),
                // Named entities
                item("entity.name.function", syntax.function.into()),
                item(
                    "entity.name.type, \
                     entity.name.class, \
                     entity.name.struct, \
                     entity.name.enum, \
                     entity.name.trait",
                    syntax.type_name.into(),
                ),
                item("entity.name.namespace", syntax.namespace.into()),
                // Built-ins
                item("support", syntax.builtin.into()),
                item("support.function", syntax.builtin_function.into()),
                item("support.type, support.class", syntax.builtin_type.into()),
                // Punctuation
                item("punctuation", syntax.punctuation.into()),
                // Markup / HTML
                item("entity.name.tag", syntax.tag.into()),
                item("entity.other.attribute-name", syntax.attribute.into()),
                item("markup.heading", syntax.heading.into()),
                item("markup.underline.link", syntax.link.into()),
                styled_item("markup.bold", syntax.markup_bold.into(), FontStyle::BOLD),
                styled_item(
                    "markup.italic",
                    syntax.markup_italic.into(),
                    FontStyle::ITALIC,
                ),
                // Diff
                item("markup.inserted", syntax.inserted.into()),
                item("markup.deleted", syntax.deleted.into()),
                // Invalid / deprecated
                item("invalid", syntax.invalid.into()),
                item("invalid.deprecated", syntax.deprecated.into()),
            ],
        }
    }
}

impl From<Rgb> for syntect::highlighting::Color {
    fn from(color: Rgb) -> Self {

        Self {
            r: color.r(),
            g: color.g(),
            b: color.b(),
            a: 255,
        }
    }
}

impl From<Rgba> for syntect::highlighting::Color {
    fn from(color: Rgba) -> Self {

        Self {
            a: color.alpha_u8(),
            ..(*color.color()).into()
        }
    }
}

impl From<Color> for syntect::highlighting::Color {
    fn from(color: Color) -> Self {

        Rgba::from(color).into()
    }
}
