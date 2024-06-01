use std::{error::Error, fmt::Display};

use ahash::HashMap;

use super::KeyBinding;

/// Stack to keep track of what path / field we are currently in
#[derive(Debug, Clone, PartialEq)]
pub(super) struct KeyPath(Vec<&'static str>);

impl KeyPath {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn new_with_toplevel(value: &'static str) -> Self {
        let mut ret = Self::new();
        ret.push(value);

        ret
    }

    /// Push a new field onto the path
    pub fn push(&mut self, value: &'static str) {
        self.0.push(value);
    }

    /// Pop the last field from the path
    pub fn pop(&mut self) -> Option<&'static str> {
        self.0.pop()
    }

    /// Convert the currently stored path to a string plus a extra value, joined via `.`
    pub fn join_with_field(&self, field: &'static str) -> String {
        let mut ret = self.0.join(".");

        ret.push('.');
        ret.push_str(field);

        ret
    }
}

// TODO: upgrade errors with what config-key has errored
// TODO: consider upgrading this with "thiserror"
/// Error for when [`Key`] parsing fails
#[derive(Debug, Clone, PartialEq)]
pub struct KeyConflictError {
    pub key_path_first: String,
    pub key_path_second: String,
    pub key: KeyBinding,
}

impl Display for KeyConflictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Key Conflict: '{}' and '{}', key: '{}'",
            self.key_path_first, self.key_path_second, self.key
        )
    }
}

impl Error for KeyConflictError {}

pub(super) type KeyHashMap = HashMap<&'static KeyBinding, &'static str>;
pub(super) type KeyHashMapOwned = HashMap<KeyBinding, String>;

pub(super) trait CheckConflict {
    /// Iterator over all the individual keys
    ///
    /// Returns `(key, key_path_name)`
    ///
    /// Only for direct keys
    fn iter(&self) -> impl Iterator<Item = (&KeyBinding, &'static str)>;
    /// Check for key conflicts with current instance and against `global_keys`
    fn check_conflict(
        &self,
        key_path: &mut KeyPath,
        global_keys: &mut KeyHashMapOwned,
    ) -> Result<(), Vec<KeyConflictError>>;
}

/// Macro to not repeat yourself writing `once(...).chain(once(...))`
///
/// Allows usage of calling one at a time:
///
/// ```
/// const_str!(1 + 1)
/// ```
///
/// or multiple at a time to even save repeated `once_chain!` invokations:
///
/// ```
/// const_str! {
///     1 + 1,
///     2 + 2,
/// }
/// ```
#[macro_export]
macro_rules! once_chain {
    (
        $first:expr
        $(
            , $second:expr
        )* $(,)?
    ) => {
        std::iter::once($first)
        $(.chain(std::iter::once($second)))*
    }
}
