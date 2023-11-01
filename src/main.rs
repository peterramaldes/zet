pub mod create;
pub mod search;

use clap::{Args, Parser};

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

    /// Search specific words on the Zettelkasten repository
    #[clap(visible_alias = "s")]
    Search(SearchArgs),
}

#[derive(Args, Debug)]
pub struct SearchArgs {
    /// The word that you are trying to find on Zettelkasten repository
    #[arg(short, long)]
    target_word: Option<String>,

    /// If this flag enable, means that the word search using the `target_word` will ignore case
    #[arg(short, long)]
    ignore_case: bool,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    return match cli.subcommand {
        Subcommand::Create => create::run(),
        Subcommand::Search(args) => search::run(args),
    };
}
