//! Adapter for DioxusLabs/components 9a758255 preview/assets/dx-components-theme.css.
//! These are upstream neutral scales, not Ferriswatch primary/secondary actions.

use crate::theme_variant::Appearance;

const COLORS: &str = "\
--primary-color:var(--fs-background);\
--primary-color-1:var(--fs-surface);\
--primary-color-2:var(--fs-alt-background);\
--primary-color-3:var(--fs-surface);\
--primary-color-4:var(--fs-hover);\
--primary-color-5:var(--fs-raised);\
--primary-color-6:var(--fs-border-muted);\
--primary-color-7:var(--fs-border);\
--secondary-color:var(--fs-text);\
--secondary-color-1:var(--fs-text);\
--secondary-color-2:var(--fs-text);\
--secondary-color-3:var(--fs-text);\
--secondary-color-4:var(--fs-text);\
--secondary-color-5:var(--fs-muted);\
--secondary-color-6:var(--fs-border);\
--focused-border-color:var(--fs-focus);\
--primary-success-color:color-mix(in srgb, var(--fs-success) 15%, var(--fs-surface));\
--secondary-success-color:var(--fs-success);\
--primary-warning-color:color-mix(in srgb, var(--fs-warning) 15%, var(--fs-surface));\
--secondary-warning-color:var(--fs-warning);\
--primary-error-color:var(--fs-error);\
--secondary-error-color:color-mix(in srgb, var(--fs-error) 85%, var(--fs-text));\
--contrast-error-color:var(--fs-background);\
--primary-info-color:color-mix(in srgb, var(--fs-info) 15%, var(--fs-surface));\
--secondary-info-color:var(--fs-info);";

pub(super) fn declarations(appearance: Appearance) -> String {
    let mode = match appearance {
        Appearance::Dark => "--dark:initial;--light: ;--dxc-dark-on:initial;--dxc-light-on: ;",
        Appearance::Light => "--dark: ;--light:initial;--dxc-dark-on: ;--dxc-light-on:initial;",
    };

    format!("{COLORS}{mode}")
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn maps_upstream_colors_and_switches_only() {
        let upstream =
            include_str!("../../../../examples/dioxus_theme/fixtures/dx-components-theme.css");

        let names: std::collections::BTreeSet<_> = upstream
            .lines()
            .map(str::trim)
            .filter(|line| line.starts_with("--"))
            .map(|line| line.split_once(':').unwrap().0)
            .collect();

        for appearance in [Appearance::Light, Appearance::Dark] {
            let css = declarations(appearance);

            let actual: std::collections::BTreeSet<_> = css
                .split(';')
                .filter(|d| !d.is_empty())
                .map(|d| d.split_once(':').unwrap().0)
                .collect();

            assert_eq!(
                actual, names,
                "every upstream color and mode switch is covered"
            );

            assert_eq!(css.split(';').filter(|d| !d.is_empty()).count(), 29);

            assert!(
                css.split(';')
                    .filter(|d| !d.is_empty())
                    .all(|d| d.starts_with("--"))
            );

            assert!(css.contains("--primary-color:var(--fs-background);"));

            assert!(css.contains("--secondary-color:var(--fs-text);"));

            assert!(css.contains("--focused-border-color:var(--fs-focus);"));

            assert_eq!(
                css.contains("--dark:initial;"),
                appearance == Appearance::Dark
            );

            assert_eq!(
                css.contains("--light:initial;"),
                appearance == Appearance::Light
            );
        }
    }
}
