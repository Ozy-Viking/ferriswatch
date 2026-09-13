//! Typed markers for selecting complete built-in palette families.

/// A family in the built-in catalogue.
pub trait PaletteFamily {
    /// Stable catalogue family ID.
    const ID: &'static str;
}

/// The catppuccin palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Catppuccin;
impl PaletteFamily for Catppuccin {
    const ID: &'static str = "catppuccin";
}

/// The everforest palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Everforest;
impl PaletteFamily for Everforest {
    const ID: &'static str = "everforest";
}

/// The gruvbox palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gruvbox;
impl PaletteFamily for Gruvbox {
    const ID: &'static str = "gruvbox";
}

/// The kanagawa palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Kanagawa;
impl PaletteFamily for Kanagawa {
    const ID: &'static str = "kanagawa";
}

/// The rose_pine palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RosePine;
impl PaletteFamily for RosePine {
    const ID: &'static str = "rose_pine";
}

/// The tokyo_night palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TokyoNight;
impl PaletteFamily for TokyoNight {
    const ID: &'static str = "tokyo_night";
}

/// The ayu palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ayu;
impl PaletteFamily for Ayu {
    const ID: &'static str = "ayu";
}

/// The base16 palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Base16;
impl PaletteFamily for Base16 {
    const ID: &'static str = "base16";
}

/// The cobalt2 palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cobalt2;
impl PaletteFamily for Cobalt2 {
    const ID: &'static str = "cobalt2";
}

/// The dracula palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dracula;
impl PaletteFamily for Dracula {
    const ID: &'static str = "dracula";
}

/// The github palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Github;
impl PaletteFamily for Github {
    const ID: &'static str = "github";
}

/// The horizon palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Horizon;
impl PaletteFamily for Horizon {
    const ID: &'static str = "horizon";
}

/// The jetbrains palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Jetbrains;
impl PaletteFamily for Jetbrains {
    const ID: &'static str = "jetbrains";
}

/// The material palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Material;
impl PaletteFamily for Material {
    const ID: &'static str = "material";
}

/// The monokai palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Monokai;
impl PaletteFamily for Monokai {
    const ID: &'static str = "monokai";
}

/// The night_owl palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NightOwl;
impl PaletteFamily for NightOwl {
    const ID: &'static str = "night_owl";
}

/// The nightfox palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Nightfox;
impl PaletteFamily for Nightfox {
    const ID: &'static str = "nightfox";
}

/// The nord palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Nord;
impl PaletteFamily for Nord {
    const ID: &'static str = "nord";
}

/// The oceanic_next palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OceanicNext;
impl PaletteFamily for OceanicNext {
    const ID: &'static str = "oceanic_next";
}

/// The one palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct One;
impl PaletteFamily for One {
    const ID: &'static str = "one";
}

/// The panda palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Panda;
impl PaletteFamily for Panda {
    const ID: &'static str = "panda";
}

/// The poimandres palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Poimandres;
impl PaletteFamily for Poimandres {
    const ID: &'static str = "poimandres";
}

/// The quiet_light palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QuietLight;
impl PaletteFamily for QuietLight {
    const ID: &'static str = "quiet_light";
}

/// The shades_of_purple palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShadesOfPurple;
impl PaletteFamily for ShadesOfPurple {
    const ID: &'static str = "shades_of_purple";
}

/// The solarized palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Solarized;
impl PaletteFamily for Solarized {
    const ID: &'static str = "solarized";
}

/// The synthwave_84 palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Synthwave84;
impl PaletteFamily for Synthwave84 {
    const ID: &'static str = "synthwave_84";
}

/// The tomorrow palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tomorrow;
impl PaletteFamily for Tomorrow {
    const ID: &'static str = "tomorrow";
}

/// The vscode palette family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vscode;
impl PaletteFamily for Vscode {
    const ID: &'static str = "vscode";
}
