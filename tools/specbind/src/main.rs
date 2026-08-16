use std::{
    io::{self, Write as _},
    process::ExitCode,
};

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "specbind",
    version,
    about = "Bind durable specifications to agent-assisted software delivery."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List or read discovered `SpecBind` artifacts.
    Artifact {
        #[command(subcommand)]
        command: ArtifactCommand,
    },
}

#[derive(Debug, Subcommand)]
enum ArtifactCommand {
    /// List recognized artifacts for one canonical Spec ID.
    List { spec: String },
    /// Read one logical artifact selector as raw UTF-8 Markdown.
    Read { spec: String, selector: String },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let start = match std::env::current_dir() {
        Ok(start) => start,
        Err(error) => {
            eprintln!("ERROR CURRENT_DIRECTORY_FAILED: {error}");
            return ExitCode::FAILURE;
        }
    };
    let output = match cli.command {
        Command::Artifact {
            command: ArtifactCommand::List { spec },
        } => specbind::cli::artifact_list(&start, &spec),
        Command::Artifact {
            command: ArtifactCommand::Read { spec, selector },
        } => specbind::cli::artifact_read(&start, &spec, &selector),
    };
    if let Err(error) = io::stdout().write_all(&output.stdout) {
        eprintln!("ERROR STDOUT_WRITE_FAILED: {error}");
        return ExitCode::FAILURE;
    }
    if let Err(error) = io::stderr().write_all(&output.stderr) {
        eprintln!("ERROR STDERR_WRITE_FAILED: {error}");
        return ExitCode::FAILURE;
    }
    if output.success {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
