use clap::{ Parser, Subcommand };

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
     
    #[arg(short, long)]
    debug: bool
}

#[derive(Subcommand)]
enum Command {
    Json {
        #[command(subcommand)]
        command: Option<JsonCommand>
    }
}

#[derive(Subcommand)]
enum JsonCommand {
    Format {

    }
}

fn main() {
    let cli = Cli::parse();
     
    if cli.debug {
        println!("Debug mode is ON");
    }

    match &cli.command {
        Some(Command::Json {command}) => {
            match *command {
                Some(JsonCommand::Format {}) => {
                    println!("Running some format command")
                }
                None => {}
            }
        }
        None => {}
    }
}
