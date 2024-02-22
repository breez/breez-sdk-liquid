use std::{path::{PathBuf, Path}, fs, str::FromStr, io};

use anyhow::Result;
use bip39::{Mnemonic, Language};

const PHRASE_FILE_NAME: &str = "phrase";
const HISTORY_FILE_NAME: &str = "history.txt";

pub(crate) struct CliPersistence {
    pub(crate) data_dir: PathBuf,
}

impl CliPersistence {
    pub(crate) fn get_or_create_mnemonic(&self) -> Result<Mnemonic> {
        let filename = Path::new(&self.data_dir).join(PHRASE_FILE_NAME);
        
        let mnemonic = match fs::read_to_string(filename.clone()) {
            Ok(phrase) => Mnemonic::from_str(&phrase).unwrap(),
            Err(e) => {
                if e.kind() != io::ErrorKind::NotFound {
                    panic!(
                        "Can't read from file: {}, err {e}",
                        filename.to_str().unwrap()
                    );
                }
                let mnemonic = Mnemonic::generate_in(Language::English, 24)?;
                fs::write(filename, mnemonic.to_string())?;
                mnemonic
            }
        };
        Ok(mnemonic)
    }

    pub(crate) fn history_file(&self) -> String {
        let path = Path::new(&self.data_dir).join(HISTORY_FILE_NAME);
        path.to_str().unwrap().to_string()
    }
}
