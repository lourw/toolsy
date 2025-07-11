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
        #[arg(short, long)]
        string: Option<String>,

        #[arg(short, long)]
        file: Option<String>,
    }
}

fn handle_json_command(json_command: &Option<JsonCommand>) {
    match json_command {
        Some(JsonCommand::Format { string, file }) => {
            println!("Running a format command with {:?} and {:?}", string, file)
        }
        None => {}
    }

}

fn main() {
    let cli = Cli::parse();
     
    if cli.debug {
        println!("Debug mode is ON");
    }

    match &cli.command {
        Some(Command::Json {command}) => {
            handle_json_command(command)
        }
        None => {}
    }
}
