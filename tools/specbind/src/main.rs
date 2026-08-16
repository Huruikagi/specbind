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
    /// Inspect the validated task plan and derived execution state.
    Tasks {
        #[command(subcommand)]
        command: TasksCommand,
    },
    /// Inspect Spec lifecycle and consistency.
    Spec {
        #[command(subcommand)]
        command: SpecCommand,
    },
}

#[derive(Debug, Subcommand)]
enum ArtifactCommand {
    /// List recognized artifacts for one canonical Spec ID.
    List { spec: String },
    /// Read one logical artifact selector as raw UTF-8 Markdown.
    Read { spec: String, selector: String },
}

#[derive(Debug, Subcommand)]
enum TasksCommand {
    /// List the ordered task hierarchy and derived progress.
    List { spec: String },
    /// Show one task's plan content and derived prerequisites.
    Show { spec: String, task_id: String },
}

#[derive(Debug, Subcommand)]
enum SpecCommand {
    /// Report lifecycle, freshness, coverage, and task progress.
    Status { spec: String },
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
        Command::Tasks {
            command: TasksCommand::List { spec },
        } => specbind::cli::tasks_list(&start, &spec),
        Command::Tasks {
            command: TasksCommand::Show { spec, task_id },
        } => specbind::cli::tasks_show(&start, &spec, &task_id),
        Command::Spec {
            command: SpecCommand::Status { spec },
        } => specbind::cli::spec_status(&start, &spec),
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
