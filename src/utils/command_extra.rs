use std::{ffi::OsStr, process::Command};

pub trait CommandExtra: Sized {
    fn with_env(self, key: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> Self;
    fn with_arg(self, arg: impl AsRef<OsStr>) -> Self;

    fn may_env(self, key: impl AsRef<OsStr>, value: Option<impl AsRef<OsStr>>) -> Self {
        if let Some(value) = value {
            self.with_env(key, value)
        } else {
            self
        }
    }

    fn arg_if(self, arg: impl AsRef<OsStr>, condition: bool) -> Self {
        if condition {
            self.with_arg(arg)
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

    fn with_arg(mut self, arg: impl AsRef<OsStr>) -> Self {
        self.arg(arg);
        self
    }
}
