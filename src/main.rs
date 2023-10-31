pub mod create;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The subcommand after `zet` that the program will run
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Parser, Debug)]
enum Subcommand {
    /// Create the Zettelkasten markdown file and open it using the default $EDITOR or `vi` if
    /// the env isn´t set.
    #[clap(visible_alias = "c")]
    Create,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    return match cli.subcommand {
        Subcommand::Create => create::run(),
    };
}
