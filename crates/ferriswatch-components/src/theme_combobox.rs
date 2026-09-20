use crate::components::{AccentSelect, ThemeSelect};
use crate::ferriswatch::dioxus::{Memory, use_theme};
use dioxus::prelude::*;

/// Joined [`ThemeSelect`] and [`AccentSelect`] controls for the nearest provider.
///
/// This widget ships no layout or chrome CSS. Pass classes for each part and
/// mount it inside [`ferriswatch::dioxus::ThemeProvider`].
///
/// When `matching_mode_only` is `true` (the default), the theme list includes
/// palettes that support the active appearance. An explicitly selected
/// out-of-mode theme stays visible because support is advisory. When `false`,
/// every configured palette is listed. Changing a theme preserves a compatible
/// accent.
///
/// # Examples
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use ferriswatch_components::components::ThemeCombobox;
///
/// fn picker() -> Element {
///     rsx! {
///         ThemeCombobox {
///             class: "theme-combobox",
///             theme_class: "theme-half",
///             accent_class: "accent-half",
///             input_class: "combobox-input",
///             list_class: "combobox-list",
///             option_class: "combobox-option",
///             empty_class: "combobox-empty",
///             matching_mode_only: false,
///         }
///     }
/// }
/// ```
#[component]
pub fn ThemeCombobox(
    /// When true, list only palettes that support the active appearance.
    #[props(default = true)]
    matching_mode_only: bool,

    /// Class for the theme half of the joined control.
    #[props(default)]
    theme_class: String,

    /// Class for the accent half of the joined control.
    #[props(default)]
    accent_class: String,

    /// Class for each primitives combobox root.
    #[props(default)]
    combobox_class: String,

    /// Class for each combobox text input.
    #[props(default)]
    input_class: String,

    /// Class for each popup list.
    #[props(default)]
    list_class: String,

    /// Class for each option.
    #[props(default)]
    option_class: String,

    /// Class for the empty-list placeholder.
    #[props(default)]
    empty_class: String,

    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let theme_state = use_theme::<Memory>();

    rsx! {
        div { role: "group", aria_label: "Theme and accent", ..attributes,
            ThemeSelect {
                matching_mode_only,
                class: theme_class,
                combobox_class: combobox_class.clone(),
                input_class: input_class.clone(),
                list_class: list_class.clone(),
                option_class: option_class.clone(),
                empty_class: empty_class.clone(),
            }
            AccentSelect {
                class: accent_class,
                combobox_class,
                input_class,
                list_class,
                option_class,
                empty_class,
            }
        }
        if let Some(message) = theme_state.last_error() {
            p { role: "alert", "{message}" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ferriswatch::{
        dioxus::ThemeProvider,
        palette::catppuccin::{Latte, Mocha, latte::Blue, mocha::Mauve},
        theme::{Appearance, Theme, ThemeSelection, config::ThemeConfig},
        theme_variant::ThemePalette,
    };

    fn mixed_config() -> ThemeConfig {
        ThemeConfig::with_default(
            Theme::new(Latte::variant::<Blue>(), Mocha::variant::<Mauve>()),
            Appearance::Dark,
        )
        .available(
            ThemeSelection::new()
                .with_palette::<Latte>()
                .with_palette::<Mocha>(),
        )
        .build()
        .unwrap()
    }

    fn app() -> Element {
        let config = mixed_config();

        rsx! {
            ThemeProvider { config,

                ThemeCombobox {
                    matching_mode_only: false,
                    class: "root",
                    theme_class: "theme",
                    accent_class: "accent",
                }
            }
        }
    }

    #[test]
    fn combobox_mounts_inside_provider() {
        let mut dom = VirtualDom::new(app);

        dom.rebuild_in_place();
    }
}
