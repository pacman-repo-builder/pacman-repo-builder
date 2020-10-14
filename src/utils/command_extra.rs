use std::{ffi::OsStr, path::Path, process::Command};

pub trait CommandExtra: Sized {
    fn with_current_dir(self, dir: impl AsRef<Path>) -> Self;
    fn with_env(self, key: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> Self;
    fn with_arg(self, arg: impl AsRef<OsStr>) -> Self;

    fn with_args<Args>(self, args: Args) -> Self
    where
        Args: IntoIterator,
        Args::Item: AsRef<OsStr>,
    {
        args.into_iter().fold(self, |cmd, arg| cmd.with_arg(arg))
    }

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
    fn with_current_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.current_dir(dir);
        self
    }

    fn with_env(mut self, key: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> Self {
        self.env(key, value);
        self
    }

    fn with_arg(mut self, arg: impl AsRef<OsStr>) -> Self {
        self.arg(arg);
        self
    }
}
