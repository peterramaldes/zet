use std::{
    fs::{self},
    io::{self, Read},
    path::PathBuf,
};

use crate::SearchArgs;

pub fn run(args: SearchArgs) -> io::Result<()> {
    println!("Search for word: '{}'", args.target_word.as_ref().unwrap());
    search_target_word(&zet::dir(), &args)
}

fn search_target_word(dir: &PathBuf, args: &SearchArgs) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_file() {
            // Check if contains the word that we are searching
            let file_path = entry.path();

            // Open the file
            let mut file = fs::File::open(&file_path)?;

            // Read the file content into a String
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            // Search for the target word in the content
            let target_word = args.target_word.as_ref().unwrap();
            if (args.ignore_case && content.to_lowercase().contains(&target_word.to_lowercase()))
                || content.contains(target_word)
            {
                println!("* {:?}", file_path);
            }
        } else {
            search_target_word(&path, args)?;
        }
    }

    Ok(())
}
