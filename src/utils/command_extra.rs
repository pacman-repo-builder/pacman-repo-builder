use std::{ffi::OsStr, process::Command};

pub trait CommandExtra<Key, Value>: Sized {
    fn with_env(self, key: Key, value: Value) -> Self;

    fn may_env(self, key: Key, value: Option<Value>) -> Self {
        if let Some(value) = value {
            self.with_env(key, value)
        } else {
            self
        }
    }
}

impl<Key, Value> CommandExtra<Key, Value> for Command
where
    Key: AsRef<OsStr>,
    Value: AsRef<OsStr>,
{
    fn with_env(mut self, key: Key, value: Value) -> Self {
        self.env(key, value);
        self
    }
}
