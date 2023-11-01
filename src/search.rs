use std::{
    fs::{self},
    io,
    path::PathBuf,
};

pub fn run() -> io::Result<()> {
    let _ = visit_dirs(&zet::dir());

    Ok(())
}

fn visit_dirs(dir: &PathBuf) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            println!("{:?}", path);
            if path.is_dir() {
                visit_dirs(&path)?;
            }
        }
    }
    Ok(())
}
