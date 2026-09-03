//! Command-line argument definitions.
//!
//! These live in the library so the accepted command graph is walkable by
//! conformance tests that verify documented invocations without running them.

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "specbind",
    version,
    about = "Bind durable specifications to agent-assisted software delivery.",
    after_help = "Feedback:\n  Report bugs or suggest improvements with `specbind feedback`."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Show where and how to report bugs or suggest improvements.
    Feedback,
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
        #[arg(long = "agent", value_parser = ["claude-code", "codex", "generic"])]
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
    /// Plan or apply removal of one selected agent integration.
    RemoveAgent {
        #[arg(value_parser = ["claude-code", "codex", "generic"])]
        agent: String,
        /// Apply the freshly recomputed guarded plan.
        #[arg(long)]
        apply: bool,
    },
    /// Plan or apply guarded removal of the project integration.
    Uninstall {
        /// Explicit policy for the complete durable `SpecBind` knowledge bundle.
        #[arg(long, value_parser = ["retain", "remove"])]
        knowledge: String,
        /// Apply the freshly recomputed guarded plan.
        #[arg(long)]
        apply: bool,
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
    /// Inspect the derived project-wide Contract dependency graph.
    Contract {
        #[command(subcommand)]
        command: ContractCommand,
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
    /// List or read project-owned shared rules.
    Rule {
        #[command(subcommand)]
        command: RuleCommand,
    },
    /// List or read project-level steering documents.
    Steering {
        #[command(subcommand)]
        command: SteeringCommand,
    },
    /// Inspect the complete supported project configuration.
    Configuration {
        #[command(subcommand)]
        command: ConfigurationCommand,
    },
    /// Prepare guarded adoption of specifications from an existing implementation.
    Adoption {
        #[command(subcommand)]
        command: AdoptionCommand,
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
    /// Plan or apply an explicit migration from a legacy product.
    Migrate {
        #[command(subcommand)]
        command: MigrateCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum ArtifactCommand {
    /// List recognized artifacts for one canonical Spec ID.
    List { spec: String },
    /// Read one logical artifact selector, optionally projected for one audience.
    Read {
        spec: String,
        selector: String,
        /// Keep only durable instructions for this use; omit for exact raw Markdown.
        #[arg(long = "for", value_parser = ["maintain", "consume"])]
        purpose: Option<String>,
    },
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
pub enum ContractCommand {
    /// Report every resolved direct Contract dependency reference.
    Graph,
    /// Report the direct provider references consumed by one Spec.
    Dependencies { spec: String },
    /// Report the direct consumer references targeting one Spec.
    Consumers { spec: String },
    /// Report File Ownership declarations matching one concrete project path.
    Owners { path: String },
}

#[derive(Debug, Subcommand)]
pub enum TemplateCommand {
    /// List recognized artifact templates in one scope.
    List {
        #[arg(value_parser = ["spec", "steering", "milestone"])]
        scope: String,
    },
    /// Read one template selector as raw UTF-8 Markdown.
    Read {
        #[arg(value_parser = ["spec", "steering", "milestone"])]
        scope: String,
        selector: String,
    },
    /// Resolve one template to its exact target path for an existing Spec.
    Resolve {
        #[arg(value_parser = ["spec"])]
        scope: String,
        spec: String,
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
    /// Read one adapter selector, optionally omitting an inactive scaffold.
    Read {
        selector: String,
        /// Return only active project guidance; omit for exact raw Markdown.
        #[arg(long = "for", value_parser = ["consume"])]
        purpose: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum RuleCommand {
    /// List every accepted shared rule and whether the project has it.
    List,
    /// Read one rule selector, optionally projected for one audience.
    Read {
        selector: String,
        /// Keep only durable instructions for this use; omit for exact raw Markdown.
        #[arg(long = "for", value_parser = ["maintain", "consume"])]
        purpose: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum SteeringCommand {
    /// List every recognized steering document.
    List,
    /// Read one steering selector, optionally projected for one audience.
    Read {
        selector: String,
        /// Keep only durable instructions for this use; omit for exact raw Markdown.
        #[arg(long = "for", value_parser = ["maintain", "consume"])]
        purpose: Option<String>,
    },
    /// Verify one materialized document against the selected Steering scaffold.
    Check {
        /// The `artifact_id` of the materialized Steering document.
        selector: String,
        /// The Steering template selector that was materialized.
        #[arg(long)]
        template: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum ConfigurationCommand {
    /// Validate and summarize every supported configuration surface.
    Show,
}

#[derive(Debug, Subcommand)]
pub enum AdoptionCommand {
    /// Return the clean committed source revision after adoption prerequisites pass.
    Preflight,
}

#[derive(Debug, Subcommand)]
pub enum SpecCommand {
    /// List every persistent Spec in the project.
    List,
    /// Report lifecycle, freshness, coverage, and task progress.
    Status {
        spec: String,
        /// Emit the command-specific machine-readable response.
        #[arg(long)]
        json: bool,
    },
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
    Status {
        /// Emit the command-specific machine-readable response.
        #[arg(long)]
        json: bool,
    },
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
    /// Finalize a reverse-establishment milestone.
    Reverse {
        #[command(subcommand)]
        command: ReverseCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum ReverseCommand {
    /// Finalize established Specs as an adopted baseline, not a release.
    Finalize {
        #[arg(long)]
        log_entries: Option<String>,
    },
    /// Abandon the active reverse milestone before urgent ordinary work.
    Abandon {
        #[arg(long)]
        milestone_id: String,
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

#[derive(Debug, Subcommand)]
pub enum MigrateCommand {
    /// Plan migration from the inherited cc-sdd project layout.
    CcSdd {
        /// Apply a freshly recomputed unambiguous plan.
        #[arg(long)]
        apply: bool,
        /// Accept an agent-authored migration resolution candidate from an external file or stdin.
        #[arg(long, value_name = "PATH_OR_STDIN", conflicts_with = "apply")]
        accept_resolution: Option<String>,
    },
}
