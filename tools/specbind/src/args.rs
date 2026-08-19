//! Command-line argument definitions.
//!
//! These live in the library so the accepted command graph is walkable by
//! conformance tests that verify documented invocations without running them.

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "specbind",
    version,
    about = "Bind durable specifications to agent-assisted software delivery."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// List or read discovered `SpecBind` artifacts.
    Artifact {
        #[command(subcommand)]
        command: ArtifactCommand,
    },
    /// Install or refresh `SpecBind` product assets in this project.
    Install {
        /// Report the plan without writing anything.
        #[arg(long)]
        dry_run: bool,
        /// Supported agent to install; repeat to select several.
        #[arg(long = "agent", value_parser = ["claude-code", "codex"])]
        agents: Vec<String>,
        /// Project-global artifact language.
        #[arg(long, value_parser = ["en", "ja"])]
        language: Option<String>,
        /// Project-root-relative specification directory.
        #[arg(long)]
        spec_dir: Option<String>,
        /// Maintain the marked `SpecBind` block in root agent instructions.
        #[arg(long)]
        project_instructions: bool,
    },
    /// Read the immutable product protocols embedded in this binary.
    Protocol {
        #[command(subcommand)]
        command: ProtocolCommand,
    },
    /// Read the structured artifact schemas embedded in this binary.
    Schema {
        #[command(subcommand)]
        command: SchemaCommand,
    },
    /// Run one deterministic read-only consistency check.
    Check {
        #[command(subcommand)]
        command: CheckCommand,
    },
    /// Read the project-owned OKF artifact templates.
    Template {
        #[command(subcommand)]
        command: TemplateCommand,
    },
    /// Inspect the validated task plan and derived execution state.
    Tasks {
        #[command(subcommand)]
        command: TasksCommand,
    },
    /// List or read project-owned operational adapters.
    Adapter {
        #[command(subcommand)]
        command: AdapterCommand,
    },
    /// List or read project-level steering documents.
    Steering {
        #[command(subcommand)]
        command: SteeringCommand,
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
pub enum ArtifactCommand {
    /// List recognized artifacts for one canonical Spec ID.
    List { spec: String },
    /// Read one logical artifact selector as raw UTF-8 Markdown.
    Read { spec: String, selector: String },
}

#[derive(Debug, Subcommand)]
pub enum ProtocolCommand {
    /// List every embedded protocol selector and its purpose.
    List,
    /// Read one protocol selector as raw UTF-8 Markdown.
    Read { selector: String },
}

#[derive(Debug, Subcommand)]
pub enum SchemaCommand {
    /// List every embedded schema selector and the artifact it governs.
    List,
    /// Read one versioned schema selector as raw JSON.
    Read { selector: String },
}

#[derive(Debug, Subcommand)]
pub enum CheckCommand {
    /// Verify Requirement existence and active Design and Task coverage.
    Traceability { spec: String },
    /// Verify the project-wide Contract graph.
    Contracts,
}

#[derive(Debug, Subcommand)]
pub enum TemplateCommand {
    /// List recognized artifact templates in one scope.
    List {
        #[arg(value_parser = ["spec", "steering"])]
        scope: String,
    },
    /// Read one template selector as raw UTF-8 Markdown.
    Read {
        #[arg(value_parser = ["spec", "steering"])]
        scope: String,
        selector: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum TasksCommand {
    /// List the ordered task hierarchy and derived progress.
    List { spec: String },
    /// Show one task's plan content and derived prerequisites.
    Show { spec: String, task_id: String },
    /// Record one executable task as completed.
    Complete { spec: String, task_id: String },
    /// Record one executable task as blocked with an explicit reason.
    Block {
        spec: String,
        task_id: String,
        #[arg(long)]
        reason: String,
    },
    /// Return one executable task to pending.
    Reopen { spec: String, task_id: String },
}

#[derive(Debug, Subcommand)]
pub enum AdapterCommand {
    /// List every accepted adapter selector and whether the project has it.
    List,
    /// Read one adapter selector as raw UTF-8 Markdown.
    Read { selector: String },
}

#[derive(Debug, Subcommand)]
pub enum SteeringCommand {
    /// List every recognized steering document.
    List,
    /// Read one steering selector as raw UTF-8 Markdown.
    Read { selector: String },
}

#[derive(Debug, Subcommand)]
pub enum SpecCommand {
    /// List every persistent Spec in the project.
    List,
    /// Report lifecycle, freshness, coverage, and task progress.
    Status { spec: String },
    /// Validate, accept, or invalidate implementation completion.
    Completion {
        #[command(subcommand)]
        command: SpecCompletionCommand,
    },
    /// Approve or invalidate the requirements gate.
    Requirements {
        #[command(subcommand)]
        command: GateCommand,
    },
    /// Approve or invalidate the design gate.
    Design {
        #[command(subcommand)]
        command: GateCommand,
    },
    /// Approve or invalidate the tasks gate.
    Tasks {
        #[command(subcommand)]
        command: GateCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum GateCommand {
    /// Record approval evidence and advance the Spec to the next state.
    Approve {
        spec: String,
        #[arg(long, value_parser = ["explicit", "delegated"])]
        approval_mode: String,
        #[arg(long)]
        delegation_workflow: Option<String>,
        /// Comma-separated canonical active Requirement IDs; requirements only.
        #[arg(long)]
        requirement_ids: Option<String>,
    },
    /// Clear this gate and its cumulative downstream evidence.
    Invalidate { spec: String },
}

#[derive(Debug, Subcommand)]
pub enum SpecCompletionCommand {
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
pub enum MilestoneCommand {
    /// Report stage, progress, actions, dependencies, and release blockers.
    Status,
    /// Write the current scope as a replacement candidate document.
    Scope {
        /// Include the complete current Markdown body in the candidate.
        #[arg(long)]
        include_body: bool,
    },
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
    /// Report or accept the milestone-owned contract review.
    Review {
        #[command(subcommand)]
        command: ReviewCommand,
    },
    /// Create the one active milestone from a confirmed scope.
    Create {
        #[arg(long)]
        scope: String,
    },
    /// Replace the active milestone's confirmed scope and body.
    UpdateScope {
        #[arg(long)]
        scope: String,
    },
    /// Replace the milestone baseline with an explicit ancestor revision.
    Rebaseline {
        #[arg(long)]
        revision: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum ReviewCommand {
    /// Report the focused contract review status for the active milestone.
    Status,
    /// Accept one strict contract review candidate document.
    Accept {
        #[arg(long)]
        candidate: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum DirectCommand {
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
pub enum ReleaseCommand {
    /// Derive current release readiness without persisting authority.
    Preflight,
    /// Finalize the complete active milestone after project release work succeeds.
    Finalize {
        #[arg(long)]
        log_entries: Option<String>,
    },
}
