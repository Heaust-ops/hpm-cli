use clap::{Parser, Subcommand};
mod config;

#[derive(Subcommand, Debug)]
enum AliasSubs {
    #[command(alias = "a")]
    /// [a]    adds an alias                   - hpm alias add <alias name> <package url>
    Add { name: String, src: String },
    /// [mod]  updates an alias                - hpm alias update/mod <alias name> <new package url>
    #[command(alias = "mod")]
    Update { name: String, src: String },
    /// [ls]   lists all available aliases     - hpm alias list
    #[command(alias = "ls")]
    List,
    /// [rm]   removes an alias                - hpm alias remove <alias name>
    #[command(alias = "rm")]
    Remove { name: String },
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// [i]    install a package   - hpm install <package url / alias>
    #[command(alias = "i")]
    Install {
        /// url or alias to install from
        url_or_alias: String,
    },
    /// [cp]   copy a snippet      - hpm copy <package url / alias>
    #[command(alias = "cp")]
    Copy {
        /// url or alias to install from
        url_or_alias: String,
    },
    /// [als]  manage your aliases - hpm alias <add / mod / ls / rm>
    #[command(alias = "als")]
    #[command(subcommand)]
    Alias(AliasSubs),
}

/// An ownership oriented package and snippets manager.
#[derive(Parser, Debug)]
#[command(name = "hpm", version, about = "Ownership-oriented package manager")]
struct Hpm {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Hpm::parse();
    let user_config = config::load_user_config();

    println!("{:?}", user_config);

    match cli.command {
        Commands::Install { url_or_alias } => {
            println!("{:?}", url_or_alias);
        }
        Commands::Copy { url_or_alias } => {
            println!("{:?}", url_or_alias);
        }
        Commands::Alias(subcommand) => match subcommand {
            AliasSubs::Add { name, src } => {
                println!("{:?}, {:?}", name, src);
            }
            AliasSubs::Update { name, src } => {
                println!("{:?}, {:?}", name, src);
            }
            AliasSubs::List => {
                println!("list");
            }
            AliasSubs::Remove { name } => {
                println!("{:?}", name);
            }
        },
    }
}
