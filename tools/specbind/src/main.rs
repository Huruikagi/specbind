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
    /// Inspect the active milestone and its derived delivery state.
    Milestone {
        #[command(subcommand)]
        command: MilestoneCommand,
    },
    /// Check or finalize the active milestone release.
    Release {
        #[command(subcommand)]
        command: ReleaseCommand,
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
    /// Validate, accept, or invalidate implementation completion.
    Completion {
        #[command(subcommand)]
        command: SpecCompletionCommand,
    },
}

#[derive(Debug, Subcommand)]
enum SpecCompletionCommand {
    /// Return the clean implementation revision ready for validation.
    Preflight { spec: String },
    /// Accept strict transient completion evidence.
    Accept {
        spec: String,
        #[arg(long)]
        evidence: String,
    },
    /// Clear completion evidence and return to implementation.
    Invalidate { spec: String },
}

#[derive(Debug, Subcommand)]
enum MilestoneCommand {
    /// Report stage, progress, actions, dependencies, and release blockers.
    Status,
    /// Bind or explicitly rebind the active milestone release label.
    BindRelease {
        version: String,
        #[arg(long)]
        rebind: bool,
    },
    /// Validate and complete a milestone-owned Direct item.
    Direct {
        #[command(subcommand)]
        command: DirectCommand,
    },
}

#[derive(Debug, Subcommand)]
enum DirectCommand {
    /// Return the clean implementation revision ready for validation.
    Preflight { direct: String },
    /// Record one Direct item completed at the validated revision.
    Complete {
        direct: String,
        #[arg(long)]
        implementation_revision: String,
    },
}

#[derive(Debug, Subcommand)]
enum ReleaseCommand {
    /// Derive current release readiness without persisting authority.
    Preflight,
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
        Command::Spec {
            command:
                SpecCommand::Completion {
                    command: SpecCompletionCommand::Preflight { spec },
                },
        } => specbind::cli::spec_completion_preflight(&start, &spec),
        Command::Spec {
            command:
                SpecCommand::Completion {
                    command: SpecCompletionCommand::Accept { spec, evidence },
                },
        } => specbind::cli::spec_completion_accept(&start, &spec, &evidence),
        Command::Spec {
            command:
                SpecCommand::Completion {
                    command: SpecCompletionCommand::Invalidate { spec },
                },
        } => specbind::cli::spec_completion_invalidate(&start, &spec),
        Command::Milestone {
            command: MilestoneCommand::Status,
        } => specbind::cli::milestone_status(&start),
        Command::Milestone {
            command: MilestoneCommand::BindRelease { version, rebind },
        } => specbind::cli::milestone_bind_release(&start, &version, rebind),
        Command::Milestone {
            command:
                MilestoneCommand::Direct {
                    command: DirectCommand::Preflight { direct },
                },
        } => specbind::cli::direct_completion_preflight(&start, &direct),
        Command::Milestone {
            command:
                MilestoneCommand::Direct {
                    command:
                        DirectCommand::Complete {
                            direct,
                            implementation_revision,
                        },
                },
        } => specbind::cli::direct_completion_complete(&start, &direct, &implementation_revision),
        Command::Release {
            command: ReleaseCommand::Preflight,
        } => specbind::cli::release_preflight(&start),
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
