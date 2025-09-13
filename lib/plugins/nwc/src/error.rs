use breez_sdk_liquid::plugin::PluginStorageError;

#[derive(thiserror::Error, Debug)]
pub enum NwcError {
    #[error("Generic error: {err}")]
    Generic { err: String },

    #[error("Could not persist data: {err}")]
    Persist { err: String },
}

impl NwcError {
    pub fn generic<T: ToString>(err: T) -> Self {
        Self::Generic {
            err: err.to_string(),
        }
    }
}

impl From<anyhow::Error> for NwcError {
    fn from(err: anyhow::Error) -> Self {
        Self::generic(err)
    }
}

impl From<serde_json::Error> for NwcError {
    fn from(err: serde_json::Error) -> Self {
        Self::generic(err)
    }
}

impl From<PluginStorageError> for NwcError {
    fn from(err: PluginStorageError) -> Self {
        Self::Persist {
            err: err.to_string(),
        }
    }
}

pub type NwcResult<T> = Result<T, NwcError>;
