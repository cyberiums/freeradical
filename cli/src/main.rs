use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "freeradical")]
#[command(about = "FreeRadical CMS CLI - Scaffold, manage, and deploy your CMS", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new FreeRadical CMS project
    Init {
        /// Project name
        name: String,
        
        /// Template to use (default, blog, ecommerce)
        #[arg(short, long, default_value = "default")]
        template: String,
    },
    
    /// Export content to JSON
    Export {
        /// Resource type (pages, modules, media)
        #[arg(short, long, default_value = "pages")]
        resource: String,
        
        /// Output file
        #[arg(short, long, default_value = "export.json")]
        output: String,
    },
    
    /// Import content from JSON
    Import {
        /// Input file
        #[arg(short, long)]
        file: String,
    },
    
    /// Run database migrations
    Migrate {
        /// Migration action (run, rollback, status)
        action: String,
    },
    
    /// Start development server
    Dev,
    
    /// Build for production
    Build,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, template } => {
            commands::init::run(name, template);
        }
        Commands::Export { resource, output } => {
            commands::export::run(resource, output);
        }
        Commands::Import { file } => {
            commands::import::run(file);
        }
        Commands::Migrate { action } => {
            commands::migrate::run(action);
        }
        Commands::Dev => {
            commands::dev::run();
        }
        Commands::Build => {
            commands::build::run();
        }
    }
}
