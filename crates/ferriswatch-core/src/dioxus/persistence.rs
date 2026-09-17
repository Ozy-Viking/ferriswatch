//! Theme snapshot persistence through [`dioxus_sdk_storage::StorageBacking`].

use super::theme_provider::ThemeState;
use crate::theme::Appearance;
use dioxus_sdk_storage::StorageBacking;
use serde::{Deserialize, Serialize};

/// Storage key written by [`ThemeStorage`] backends.

pub const KEY: &str = "ferriswatch.theme.v1";

/// Persisted mode, palette id, and optional accent.
///
/// This is the serializable form of the active selection. [`ThemeState`] is a
/// live Dioxus handle and cannot be stored directly.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]

pub struct ThemeSnapshot {
    /// Active appearance.
    pub mode: Appearance,
    /// Canonical palette id.
    pub id: String,
    /// Explicit accent id, or `None` for the palette default.
    pub accent: Option<String>,
}

/// Load and save the active theme selection.
///
/// [`Memory`] is a no-op. Any [`StorageBacking`] with a [`String`] key, such as
/// [`dioxus_sdk_storage::LocalStorage`] or [`dioxus_sdk_storage::SessionStorage`],
/// stores a [`ThemeSnapshot`] under [`KEY`].

pub trait ThemeStorage {
    /// Restores a previously saved selection, if one exists.

    fn load() -> Option<ThemeSnapshot> {
        None
    }

    /// Writes the live selection. [`Memory`] ignores the argument.

    fn save(_theme: &ThemeState) {}
}

/// Process-local storage that never reads or writes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]

pub struct Memory;

impl ThemeStorage for Memory {}

/// Uses [`KEY`] for the storage key.

impl<S> ThemeStorage for S
where
    S: StorageBacking<Key = String>,
{
    fn load() -> Option<ThemeSnapshot> {
        S::get(&KEY.to_string())
    }

    fn save(theme: &ThemeState) {
        S::set(KEY.to_string(), &theme.snapshot());
    }
}
