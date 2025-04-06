use anyhow::anyhow;
use std::path::Path;

pub trait PathExt {
    fn to_str_safe(&self) -> anyhow::Result<&str>;
}

impl PathExt for Path {
    fn to_str_safe(&self) -> anyhow::Result<&str> {
        self.to_str()
            .ok_or_else(|| anyhow!("Invalid UTF-8 sequence in path: {:?}", self))
    }
}
