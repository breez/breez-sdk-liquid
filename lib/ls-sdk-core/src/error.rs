use anyhow::Error;

#[macro_export]
macro_rules! ensure_sdk {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}

// TODO Unify error enum
#[derive(Debug, thiserror::Error)]
pub enum LsSdkError {
    #[error("Error: {err}")]
    Generic { err: String },
}

impl From<anyhow::Error> for LsSdkError {
    fn from(e: Error) -> Self {
        LsSdkError::Generic { err: e.to_string() }
    }
}
