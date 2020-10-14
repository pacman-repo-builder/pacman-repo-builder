use command_extra::CommandExtra;
use std::{ffi::OsStr, process::Command};

pub trait CommandUtils: CommandExtra {
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

impl CommandUtils for Command {}
