pub mod create;

fn main() -> std::io::Result<()> {
    create::run()?;
    Ok(())
}
