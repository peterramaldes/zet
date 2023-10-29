use std::{env, fs, io};

fn main() -> io::Result<()> {
    let path = get_path_from_files();

    // Create the path if doesn't exists, thrown an error if cannot
    fs::create_dir_all(path)?;

    Ok(())
}

/// Get path with this precendece:
///
/// 1. $ZET_PATH
/// 2. $HOME/.config/zet
///
/// This function is not portable, the .config folder will not work on windows
fn get_path_from_files() -> String {
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

    path
}
