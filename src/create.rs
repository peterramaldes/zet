use chrono::Utc;
use std::{
    env,
    fs::{self, File},
    io,
    process::Command,
};

pub fn run() -> io::Result<()> {
    let path = zet::dir();

    // Create the path if doesn't exists, thrown an error if cannot
    fs::create_dir_all(&path)?;

    // Create dir if doesn´t exists
    let today = Utc::now();
    let folder_name = today.date_naive().format("%Y%m%d");
    let folder_path = format!(
        "{}/{}",
        path.into_os_string().into_string().unwrap(),
        folder_name
    );
    let _ = fs::create_dir(&folder_path);

    // Create the file
    let filename = format!("{}.md", today.format("%Y%m%d%H%M%S"));
    let file_path = format!("{folder_path}/{filename}");
    let _ = File::create(&file_path)?; // TODO: can we use this return to open the file?

    // Open the file using the $EDITOR env, using the `vi` if doesn´t set
    let editor = env::var("EDITOR").unwrap_or_else(|_e| String::from("vi"));
    let _ = Command::new(editor).arg(file_path).status()?;

    Ok(())
}
