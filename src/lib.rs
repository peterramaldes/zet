use std::{
    env,
    path::{Path, PathBuf},
};

/// Retrieve the current dir for Zettelkasten notes
///
/// 1. $ZET_PATH
/// 2. $HOME/.config/zet
///
/// This function is not portable, the .config folder will not work on windows
pub fn dir() -> PathBuf {
    let mut path = match env::var("ZET_PATH") {
        Ok(val) => String::from(val),
        Err(_e) => "".to_string(),
    };

    if path.is_empty() {
        let home = match env::var("HOME") {
            Ok(val) => String::from(val),
            Err(_e) => panic!("doesn't have $HOME env set"), // TODO: this should not be a problem
                                                             // for windows :(
        };

        path = format!("{home}/.config/zet");
    }

    return Path::new(&path).to_path_buf();
}
