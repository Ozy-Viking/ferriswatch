use viking_macros::{EnumAsStr, EnumDisplay, EnumVec};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    name: ThemeName,
    /// Override the name with your own
    name_override: Option<String>,
}

impl Theme {
    pub fn new(name: ThemeName, name_override: Option<String>) -> Result<Self, ThemeError> {
        if name.is_custom() && name_override.is_none() {
            return Err(ThemeError::NoCustomThemeNameSet);
        }
        Ok(Self {
            name,
            name_override,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumDisplay, EnumVec, EnumAsStr)]
pub enum ThemeName {
    Cattpuchin,
    Tokio,
    Custom,
}

impl ThemeName {
    /// Returns `true` if the theme name is [`Custom`].
    ///
    /// [`Custom`]: ThemeName::Custom
    #[must_use]
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom)
    }
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ThemeError {
    #[error("No name override exists for custom theme name")]
    NoCustomThemeNameSet,
}
