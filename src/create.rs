use chrono::Utc;
use std::{
    env,
    fs::{self, File},
    io,
    os::unix::process::CommandExt,
    process::Command,
};

pub fn run() -> io::Result<()> {
    let path = get_path_from_files();

    // Create the path if doesn't exists, thrown an error if cannot
    fs::create_dir_all(&path)?;

    // Create dir if doesn´t exists
    let today = Utc::now();
    let folder_name = today.date_naive().format("%Y%m%d");
    let folder_path = format!("{path}/{folder_name}");
    let _ = fs::create_dir(&folder_path);

    // Create the file
    let filename = format!("{}.md", today.format("%Y%m%d%H%M%S"));
    let file_path = format!("{folder_path}/{filename}");
    let _ = File::create(&file_path)?; // TODO: can we use this return to open the file?

    // Open the file using the $EDITOR env, using the `vi` if doesn´t set
    let editor = env::var("EDITOR").unwrap_or_else(|_e| String::from("vi"));
    Command::new(editor).arg(file_path).exec();

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
