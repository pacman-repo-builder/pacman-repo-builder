use std::{ffi::OsStr, process::Command};

pub trait CommandExtra: Sized {
    fn with_env(self, key: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> Self;

    fn may_env(self, key: impl AsRef<OsStr>, value: Option<impl AsRef<OsStr>>) -> Self {
        if let Some(value) = value {
            self.with_env(key, value)
        } else {
            self
        }
    }
}

impl CommandExtra for Command {
    fn with_env(mut self, key: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> Self {
        self.env(key, value);
        self
    }
}
