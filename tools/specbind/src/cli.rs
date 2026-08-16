//! Concise text CLI execution and stream routing.

use std::{
    fmt::Write as _,
    fs,
    io::{self, Read as _},
    path::Path,
};

use crate::{
    approval,
    artifacts::{self, Artifact, DiscoveryIssue},
    completion::{self, CompletionIssue},
    config,
    contract_graph::{self, GraphIssueSeverity},
    cross_spec_review::{self, ReviewFreshnessStatus, ReviewIssue},
    milestone::{self, MilestoneIssue},
    milestone_status::{self, MilestoneHealth, MilestoneStatusModel},
    release_finalize::{self, FinalizeIssue},
    release_readiness::{self, MutationTargetState, ReleaseDiagnostic},
    spec_status::{self, ConsistencyHealth, SpecStatusModel},
    task_read_model::{GroupView, TaskPlanItemView, TaskReadModel, TaskStatus, TaskView},
    template,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub success: bool,
}

impl CommandOutput {
    fn success(stdout: Vec<u8>) -> Self {
        Self {
            stdout,
            stderr: vec![],
            success: true,
        }
    }

    fn failure(code: &str, message: impl AsRef<str>, details: Vec<String>) -> Self {
        let mut stderr = format!("ERROR {code}: {}\n", escape(message.as_ref()));
        for detail in details {
            stderr.push_str("  ");
            stderr.push_str(&escape(&detail));
            stderr.push('\n');
        }
        Self {
            stdout: vec![],
            stderr: stderr.into_bytes(),
            success: false,
        }
    }

    fn no_change(code: &str, message: &str) -> Self {
        Self::success(format!("NO_CHANGE {code}: {message}\n").into_bytes())
    }
}

#[must_use]
pub fn artifact_list(start: &Path, canonical_spec: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let inventory = artifacts::discover_spec(&paths.specbind_root, canonical_spec);
    if !inventory.issues.is_empty() {
        let mut details = inventory
            .artifacts
            .iter()
            .map(render_artifact)
            .collect::<Vec<_>>();
        details.extend(inventory.issues.iter().map(render_issue));
        return CommandOutput::failure(
            "ARTIFACT_LIST_FAILED",
            format!("Artifact inventory for spec {canonical_spec} has diagnostics."),
            details,
        );
    }
    let mut output = format!(
        "OK ARTIFACT_LISTED: Found {} recognized artifact(s) for spec {}.\n",
        inventory.artifacts.len(),
        escape(canonical_spec)
    );
    for artifact in &inventory.artifacts {
        output.push_str("  ");
        output.push_str(&render_artifact(artifact));
        output.push('\n');
    }
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn artifact_read(start: &Path, canonical_spec: &str, selector: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let inventory = artifacts::discover_spec(&paths.specbind_root, canonical_spec);
    let Some(artifact) = inventory
        .artifacts
        .iter()
        .find(|artifact| artifact.selector == selector)
    else {
        return CommandOutput::failure(
            "ARTIFACT_SELECTOR_NOT_FOUND",
            format!("Selector {selector} does not resolve for spec {canonical_spec}."),
            inventory.issues.iter().map(render_issue).collect(),
        );
    };
    let selected_issues = inventory
        .issues
        .iter()
        .filter(|issue| issue.path.as_ref() == Some(&artifact.path))
        .map(render_issue)
        .collect::<Vec<_>>();
    if !selected_issues.is_empty() {
        return CommandOutput::failure(
            "ARTIFACT_READ_INVALID",
            format!("Selector {selector} has profile or content diagnostics."),
            inventory.issues.iter().map(render_issue).collect(),
        );
    }
    let path = paths.specbind_root.join(artifact.path.as_std_path());
    if !fs::symlink_metadata(&path)
        .is_ok_and(|metadata| metadata.is_file() && !metadata.file_type().is_symlink())
    {
        return CommandOutput::failure(
            "ARTIFACT_READ_TARGET_INVALID",
            "Resolved artifact is no longer a regular non-symlink file.",
            vec![],
        );
    }
    match fs::read(path) {
        Ok(bytes) if std::str::from_utf8(&bytes).is_ok() => {
            let mut stderr = String::new();
            for issue in &inventory.issues {
                stderr.push_str("  ");
                stderr.push_str(&render_issue(issue));
                stderr.push('\n');
            }
            CommandOutput {
                stdout: bytes,
                stderr: stderr.into_bytes(),
                success: true,
            }
        }
        Ok(_) => CommandOutput::failure(
            "ARTIFACT_READ_NOT_UTF8",
            "Resolved artifact is not valid UTF-8.",
            vec![],
        ),
        Err(error) => CommandOutput::failure("ARTIFACT_READ_FAILED", error.to_string(), vec![]),
    }
}

#[must_use]
pub fn check_traceability(start: &Path, canonical_spec: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let resolution = artifacts::resolve_traceability(&paths.specbind_root, canonical_spec);
    let mut details = resolution
        .inventory
        .issues
        .iter()
        .map(render_issue)
        .collect::<Vec<_>>();
    let Some(report) = resolution.report else {
        return CommandOutput::failure(
            "TRACEABILITY_FAILED",
            format!("Cannot verify traceability for spec {canonical_spec}."),
            details,
        );
    };
    details.extend(report.issues.iter().map(render_traceability_issue));
    if !details.is_empty() {
        return CommandOutput::failure(
            "TRACEABILITY_FAILED",
            format!("Traceability for spec {canonical_spec} has diagnostics."),
            details,
        );
    }
    let mut output = format!(
        "OK TRACEABILITY_VERIFIED: Verified traceability for spec {}.\n",
        escape(canonical_spec)
    );
    push_field(
        &mut output,
        "Requirements",
        &report.requirement_ids.len().to_string(),
    );
    match &report.active_requirement_ids {
        Some(active) => {
            let active_set = active.iter().collect::<std::collections::BTreeSet<_>>();
            let design = report
                .design_requirement_ids
                .iter()
                .filter(|id| active_set.contains(id))
                .count();
            let tasks = report
                .task_requirement_ids
                .iter()
                .filter(|id| active_set.contains(id))
                .count();
            push_field(
                &mut output,
                "Active requirement IDs",
                &active.len().to_string(),
            );
            push_field(
                &mut output,
                "Design coverage",
                &format!("{design}/{}", active.len()),
            );
            push_field(
                &mut output,
                "Task coverage",
                &format!(
                    "{tasks}/{} ({})",
                    active.len(),
                    if report.tasks_required {
                        "required"
                    } else {
                        "not required"
                    }
                ),
            );
        }
        None => push_field(&mut output, "Active requirement IDs", "none"),
    }
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn check_contracts(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let graph = contract_graph::resolve(&paths.specbind_root);
    let mut errors = graph
        .project_issues
        .iter()
        .map(render_issue)
        .collect::<Vec<_>>();
    for (spec, inventory) in &graph.inventories {
        errors.extend(
            inventory
                .issues
                .iter()
                .map(|issue| format!("{} spec {spec}", render_issue(issue))),
        );
    }
    errors.extend(
        graph
            .report
            .issues
            .iter()
            .filter(|issue| issue.severity == GraphIssueSeverity::Error)
            .map(render_graph_issue),
    );
    if !errors.is_empty() {
        return CommandOutput::failure(
            "CONTRACTS_FAILED",
            "Contract graph has structural diagnostics.",
            errors,
        );
    }
    let warnings = graph
        .report
        .issues
        .iter()
        .filter(|issue| issue.severity == GraphIssueSeverity::Warning)
        .map(render_graph_issue)
        .collect::<Vec<_>>();
    let mut output = format!(
        "OK CONTRACTS_VERIFIED: Verified {} contract(s) and {} dependency reference(s).\n",
        graph.report.contracts.len(),
        graph.report.dependencies.len()
    );
    push_field(
        &mut output,
        "Ownership findings",
        &graph.report.ownership_findings.len().to_string(),
    );
    push_field(
        &mut output,
        "Dependency cycles",
        &graph.report.dependency_cycles.len().to_string(),
    );
    if warnings.is_empty() {
        push_field(&mut output, "Warnings", "none");
    } else {
        output.push_str("  Warnings:\n");
        for warning in warnings {
            writeln!(output, "    - {warning}").expect("writing to a String cannot fail");
        }
    }
    CommandOutput::success(output.into_bytes())
}

fn render_traceability_issue(issue: &crate::traceability::TraceabilityIssue) -> String {
    let source = issue
        .source
        .as_ref()
        .map_or_else(String::new, |source| format!(" {}:", escape(source)));
    format!("{}{source} {}", issue.code, escape(&issue.message))
}

fn render_graph_issue(issue: &contract_graph::ContractGraphIssue) -> String {
    let source = issue
        .source
        .as_ref()
        .map_or_else(String::new, |source| format!(" {}:", escape(source)));
    format!("{}{source} {}", issue.code, escape(&issue.message))
}

#[must_use]
pub fn template_list_spec(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let inventory = template::discover_spec_templates(&paths.specbind_root, paths.language);
    if !inventory.issues.is_empty() {
        let mut details = inventory
            .templates
            .iter()
            .map(render_template)
            .collect::<Vec<_>>();
        details.extend(inventory.issues.iter().map(render_issue));
        return CommandOutput::failure(
            "TEMPLATE_LIST_FAILED",
            "Spec template inventory has diagnostics.",
            details,
        );
    }
    let mut output = format!(
        "OK TEMPLATE_LISTED: Found {} recognized spec template(s).\n",
        inventory.templates.len()
    );
    for template in &inventory.templates {
        output.push_str("  ");
        output.push_str(&render_template(template));
        output.push('\n');
    }
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn template_read_spec(start: &Path, selector: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match template::read_spec_template(&paths.specbind_root, paths.language, selector) {
        Ok((content, inventory)) => {
            let mut stderr = String::new();
            for issue in &inventory.issues {
                stderr.push_str("  ");
                stderr.push_str(&render_issue(issue));
                stderr.push('\n');
            }
            CommandOutput {
                stdout: content.into_bytes(),
                stderr: stderr.into_bytes(),
                success: true,
            }
        }
        Err(inventory) => {
            let resolved = inventory
                .templates
                .iter()
                .any(|template| template.selector == selector);
            let code = if resolved {
                "TEMPLATE_READ_FAILED"
            } else {
                "TEMPLATE_SELECTOR_NOT_FOUND"
            };
            CommandOutput::failure(
                code,
                format!("Selector {selector} does not resolve to a readable spec template."),
                inventory.issues.iter().map(render_issue).collect(),
            )
        }
    }
}

fn render_template(template: &template::Template) -> String {
    let mut output = format!(
        "selector={} source={} type=\"{}\"",
        escape(&template.selector),
        template.source.name(),
        escape(&template.artifact_type)
    );
    if let Some(artifact_id) = &template.artifact_id {
        output.push_str(" artifact_id=");
        output.push_str(&escape(artifact_id));
    }
    write!(
        output,
        " template_path={} output_path={}",
        escape(template.template_path.as_str()),
        escape(template.output_path.as_str())
    )
    .expect("writing to a String cannot fail");
    output
}

#[must_use]
pub fn tasks_list(start: &Path, canonical_spec: &str) -> CommandOutput {
    let model = match load_task_model(start, canonical_spec) {
        Ok(model) => model,
        Err(output) => return output,
    };
    let mut output = format!(
        "OK TASKS_LISTED: Listed {} task(s) for spec {} ({} completed, {} pending, {} blocked).\n",
        model.total(),
        escape(canonical_spec),
        model.completed,
        model.pending,
        model.blocked
    );
    for item in &model.items {
        match item {
            TaskPlanItemView::Task(task) => {
                output.push_str("  ");
                output.push_str(&render_task_summary(task));
                output.push('\n');
            }
            TaskPlanItemView::Group(group) => render_group(group, &mut output),
        }
    }
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn tasks_show(start: &Path, canonical_spec: &str, task_id: &str) -> CommandOutput {
    let model = match load_task_model(start, canonical_spec) {
        Ok(model) => model,
        Err(output) => return output,
    };
    let Some(task) = model.task(task_id) else {
        return CommandOutput::failure(
            "TASK_NOT_FOUND",
            format!("Task {task_id} does not exist in spec {canonical_spec}."),
            vec![],
        );
    };
    let mut output = format!(
        "OK TASK_SHOWN: Found task {} in spec {}.\n",
        escape(&task.id),
        escape(canonical_spec)
    );
    push_field(&mut output, "Status", &render_status(task));
    push_field(&mut output, "Title", &task.title);
    push_field(
        &mut output,
        "Group",
        &task.group.as_ref().map_or_else(
            || "none".to_owned(),
            |(id, title)| format!("{} {}", escape(id), escape(title)),
        ),
    );
    push_list(&mut output, "Details", &task.details);
    push_inline_list(&mut output, "Requirement IDs", &task.requirement_ids);
    push_inline_list(&mut output, "Boundaries", &task.boundaries);
    push_inline_list(&mut output, "Contracts", &task.contracts);
    push_inline_list(
        &mut output,
        "Explicit prerequisites",
        &task.explicit_dependencies,
    );
    push_inline_list(
        &mut output,
        "Effective prerequisites",
        &task.effective_dependencies,
    );
    push_field(
        &mut output,
        "Blocker",
        task.blocked_reason.as_deref().unwrap_or("none"),
    );
    push_list(
        &mut output,
        "Completion criteria",
        &task.completion_criteria,
    );
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn spec_status(start: &Path, canonical_spec: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let model =
        match spec_status::resolve(&paths.project_root, &paths.specbind_root, canonical_spec) {
            Ok(model) => model,
            Err(error) => {
                return CommandOutput::failure(
                    "SPEC_STATUS_FAILED",
                    format!("Cannot report status for spec {canonical_spec}."),
                    error.issues.iter().map(render_issue).collect(),
                );
            }
        };
    render_spec_status(canonical_spec, &model)
}

#[must_use]
pub fn spec_completion_preflight(start: &Path, canonical_spec: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match completion::spec_preflight(
        &paths.project_root,
        &paths.specbind_root,
        canonical_spec,
    ) {
        Ok(completion::SpecPreflightOutcome::Ready {
            implementation_revision,
        }) => CommandOutput::success(
            format!(
                "OK SPEC_COMPLETION_PREFLIGHT_READY: Spec {} is ready for completion validation.\n  Implementation revision: {}\n",
                escape(canonical_spec),
                implementation_revision
            )
            .into_bytes(),
        ),
        Ok(completion::SpecPreflightOutcome::AlreadyAccepted { .. }) => CommandOutput::no_change(
            "SPEC_COMPLETION_ALREADY_ACCEPTED",
            &format!("Spec {} already has fresh completion evidence.", escape(canonical_spec)),
        ),
        Err(error) => render_completion_failure("Cannot begin Spec completion validation.", error),
    }
}

#[must_use]
pub fn spec_completion_accept(
    start: &Path,
    canonical_spec: &str,
    evidence_source: &str,
) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let evidence = match read_external_json(start, &paths.project_root, evidence_source) {
        Ok(evidence) => evidence,
        Err(output) => return output,
    };
    match completion::spec_accept(
        &paths.project_root,
        &paths.specbind_root,
        canonical_spec,
        &evidence,
    ) {
        Ok(completion::SpecAcceptOutcome::Accepted {
            implementation_revision,
        }) => CommandOutput::success(
            format!(
                "OK SPEC_COMPLETION_ACCEPTED: Accepted completion for spec {}.\n  Implementation revision: {}\n",
                escape(canonical_spec),
                implementation_revision
            )
            .into_bytes(),
        ),
        Ok(completion::SpecAcceptOutcome::AlreadyAccepted { .. }) => CommandOutput::no_change(
            "SPEC_COMPLETION_ALREADY_ACCEPTED",
            &format!("Spec {} already has identical fresh completion evidence.", escape(canonical_spec)),
        ),
        Err(error) => render_completion_failure("Cannot accept Spec completion evidence.", error),
    }
}

#[must_use]
pub fn spec_completion_invalidate(start: &Path, canonical_spec: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match completion::spec_invalidate(&paths.project_root, &paths.specbind_root, canonical_spec) {
        Ok(completion::SpecInvalidateOutcome::Invalidated) => CommandOutput::success(
            format!(
                "OK SPEC_COMPLETION_INVALIDATED: Invalidated completion for spec {}.\n",
                escape(canonical_spec)
            )
            .into_bytes(),
        ),
        Ok(completion::SpecInvalidateOutcome::NoChange) => CommandOutput::no_change(
            "SPEC_COMPLETION_NOT_ACCEPTED",
            &format!(
                "Spec {} has no accepted completion to invalidate.",
                escape(canonical_spec)
            ),
        ),
        Err(error) => render_completion_failure("Cannot invalidate Spec completion.", error),
    }
}

#[must_use]
pub fn direct_completion_preflight(start: &Path, canonical_direct: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match completion::direct_preflight(
        &paths.project_root,
        &paths.specbind_root,
        canonical_direct,
    ) {
        Ok(completion::DirectPreflightOutcome::Ready {
            implementation_revision,
        }) => CommandOutput::success(
            format!(
                "OK DIRECT_COMPLETION_PREFLIGHT_READY: Direct item {} is ready for completion validation.\n  Implementation revision: {}\n",
                escape(canonical_direct),
                implementation_revision
            )
            .into_bytes(),
        ),
        Ok(completion::DirectPreflightOutcome::AlreadyCompleted) => CommandOutput::no_change(
            "DIRECT_COMPLETION_ALREADY_RECORDED",
            &format!("Direct item {} is already completed.", escape(canonical_direct)),
        ),
        Err(error) => render_completion_failure("Cannot begin Direct completion validation.", error),
    }
}

#[must_use]
pub fn direct_completion_complete(
    start: &Path,
    canonical_direct: &str,
    implementation_revision: &str,
) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match completion::direct_complete(
        &paths.project_root,
        &paths.specbind_root,
        canonical_direct,
        implementation_revision,
    ) {
        Ok(completion::DirectCompleteOutcome::Recorded) => CommandOutput::success(
            format!(
                "OK DIRECT_COMPLETION_RECORDED: Recorded completion for Direct item {}.\n",
                escape(canonical_direct)
            )
            .into_bytes(),
        ),
        Ok(completion::DirectCompleteOutcome::AlreadyCompleted) => CommandOutput::no_change(
            "DIRECT_COMPLETION_ALREADY_RECORDED",
            &format!(
                "Direct item {} is already completed.",
                escape(canonical_direct)
            ),
        ),
        Err(error) => render_completion_failure("Cannot record Direct completion.", error),
    }
}

/// One transient external command input and its stable diagnostic vocabulary.
///
/// Every caller shares the same safety boundary: `-` reads standard input, a
/// path must be an ordinary non-symlink file, and the content must be UTF-8.
/// Inputs that carry authority over persisted state additionally require a
/// repository-external path so the worktree cannot supply its own evidence.
struct ExternalInputSpec {
    read_failed: &'static str,
    target_invalid: &'static str,
    /// Subject phrase used for the standard-input diagnostic.
    stdin_subject: &'static str,
    /// Subject phrase used inside a sentence.
    subject: &'static str,
    /// Subject phrase used to start a sentence.
    capitalized: &'static str,
    require_external: bool,
}

struct ExternalInputError {
    code: &'static str,
    message: String,
}

const COMPLETION_EVIDENCE_INPUT: ExternalInputSpec = ExternalInputSpec {
    read_failed: "COMPLETION_EVIDENCE_READ_FAILED",
    target_invalid: "COMPLETION_EVIDENCE_TARGET_INVALID",
    stdin_subject: "completion evidence",
    subject: "completion evidence",
    capitalized: "Completion evidence",
    require_external: true,
};

const LOG_ENTRIES_INPUT: ExternalInputSpec = ExternalInputSpec {
    read_failed: "LOG_INPUT_READ_FAILED",
    target_invalid: "LOG_INPUT_TARGET_INVALID",
    stdin_subject: "log entries",
    subject: "log-entry input",
    capitalized: "Log-entry input",
    require_external: false,
};

const SCOPE_INPUT: ExternalInputSpec = ExternalInputSpec {
    read_failed: "MILESTONE_SCOPE_READ_FAILED",
    target_invalid: "MILESTONE_SCOPE_TARGET_INVALID",
    stdin_subject: "milestone scope",
    subject: "milestone scope",
    capitalized: "Milestone scope",
    require_external: true,
};

const REVIEW_CANDIDATE_INPUT: ExternalInputSpec = ExternalInputSpec {
    read_failed: "MILESTONE_REVIEW_CANDIDATE_READ_FAILED",
    target_invalid: "MILESTONE_REVIEW_CANDIDATE_TARGET_INVALID",
    stdin_subject: "review candidate",
    subject: "review candidate",
    capitalized: "Review candidate",
    require_external: true,
};

fn read_external_input(
    spec: &ExternalInputSpec,
    start: &Path,
    project_root: &Path,
    source: &str,
) -> Result<String, ExternalInputError> {
    let read_failed = |message: String| ExternalInputError {
        code: spec.read_failed,
        message,
    };
    let target_invalid = |message: String| ExternalInputError {
        code: spec.target_invalid,
        message,
    };
    if source == "-" {
        let mut input = String::new();
        return io::stdin()
            .read_to_string(&mut input)
            .map(|_| input)
            .map_err(|error| {
                read_failed(format!(
                    "Cannot read {} from stdin: {error}",
                    spec.stdin_subject
                ))
            });
    }
    let requested = start.join(source);
    let metadata = fs::symlink_metadata(&requested)
        .map_err(|error| read_failed(format!("Cannot inspect {}: {error}", spec.subject)))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(target_invalid(format!(
            "{} must be a regular non-symlink file.",
            spec.capitalized
        )));
    }
    let source_path = if spec.require_external {
        let canonical = requested
            .canonicalize()
            .map_err(|error| read_failed(format!("Cannot resolve {}: {error}", spec.subject)))?;
        let canonical_project = project_root
            .canonicalize()
            .map_err(|error| read_failed(format!("Cannot resolve project root: {error}")))?;
        if canonical.starts_with(canonical_project) {
            return Err(target_invalid(format!(
                "{} file must be outside the project worktree.",
                spec.capitalized
            )));
        }
        canonical
    } else {
        requested
    };
    fs::read_to_string(source_path)
        .map_err(|error| read_failed(format!("Cannot read {} as UTF-8: {error}", spec.subject)))
}

fn read_external_json(
    start: &Path,
    project_root: &Path,
    source: &str,
) -> Result<String, CommandOutput> {
    read_external_input(&COMPLETION_EVIDENCE_INPUT, start, project_root, source)
        .map_err(|error| CommandOutput::failure(error.code, error.message, vec![]))
}

fn render_completion_failure(message: &str, error: completion::CompletionIssues) -> CommandOutput {
    let mut issues = error.issues.into_iter();
    let Some(first) = issues.next() else {
        return CommandOutput::failure("COMPLETION_FAILED", message, vec![]);
    };
    let mut details = vec![render_completion_issue(&first)];
    details.extend(issues.map(|issue| render_completion_issue(&issue)));
    CommandOutput::failure(first.code, message, details)
}

fn render_completion_issue(issue: &CompletionIssue) -> String {
    let path = issue
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path)));
    format!("{}{path} {}", issue.code, escape(&issue.message))
}

#[must_use]
pub fn milestone_bind_release(start: &Path, version: &str, rebind: bool) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match milestone::bind_release(
        &paths.project_root,
        &paths.specbind_root,
        version,
        rebind,
    ) {
        Ok(milestone::BindReleaseOutcome::Bound {
            milestone_id,
            version,
            targets,
        }) => CommandOutput::success(
            format!(
                "OK RELEASE_BOUND: Bound milestone {} to release {}.\n  Roadmap archive: {}\n  Cross-spec review archive: {}\n",
                escape(&milestone_id),
                escape(&version),
                escape(&targets.roadmap),
                escape(&targets.cross_spec_review)
            )
            .into_bytes(),
        ),
        Ok(milestone::BindReleaseOutcome::Rebound {
            milestone_id,
            previous,
            version,
            targets,
        }) => CommandOutput::success(
            format!(
                "OK RELEASE_REBOUND: Rebound milestone {} from release {} to {}.\n  Roadmap archive: {}\n  Cross-spec review archive: {}\n",
                escape(&milestone_id),
                escape(&previous),
                escape(&version),
                escape(&targets.roadmap),
                escape(&targets.cross_spec_review)
            )
            .into_bytes(),
        ),
        Ok(milestone::BindReleaseOutcome::AlreadyBound {
            milestone_id,
            version,
        }) => CommandOutput::no_change(
            "RELEASE_ALREADY_BOUND",
            &format!(
                "Milestone {} is already bound to release {}.",
                escape(&milestone_id),
                escape(&version)
            ),
        ),
        Err(error) => render_milestone_mutation_failure("Cannot bind milestone release.", error),
    }
}

#[must_use]
pub fn milestone_create(start: &Path, scope_source: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let scope = match read_scope_document(start, &paths.project_root, scope_source) {
        Ok(scope) => scope,
        Err(error) => {
            return CommandOutput::failure(
                "MILESTONE_CREATE_FAILED",
                "Cannot create the active milestone.",
                vec![format!("{} {}", error.code, escape(&error.message))],
            );
        }
    };
    match milestone::create(&paths.project_root, &paths.specbind_root, &scope) {
        Ok(milestone::CreateOutcome::Created {
            milestone_id,
            baseline_revision,
            counts,
        }) => {
            let mut output = format!(
                "OK MILESTONE_CREATED: Created milestone {}.\n",
                escape(&milestone_id)
            );
            push_field(&mut output, "Baseline revision", &baseline_revision);
            push_scope_counts(&mut output, &counts);
            CommandOutput::success(output.into_bytes())
        }
        Err(error) => render_milestone_failure(
            "MILESTONE_CREATE_FAILED",
            "Cannot create the active milestone.",
            &error,
        ),
    }
}

#[must_use]
pub fn milestone_update_scope(start: &Path, scope_source: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let scope = match read_scope_document(start, &paths.project_root, scope_source) {
        Ok(scope) => scope,
        Err(error) => {
            return CommandOutput::failure(
                "MILESTONE_SCOPE_UPDATE_FAILED",
                "Cannot update the milestone scope.",
                vec![format!("{} {}", error.code, escape(&error.message))],
            );
        }
    };
    match milestone::update_scope(&paths.project_root, &paths.specbind_root, &scope) {
        Ok(milestone::ScopeUpdateOutcome::Updated {
            milestone_id,
            counts,
            review_removed,
        }) => {
            let mut output = format!(
                "OK MILESTONE_SCOPE_UPDATED: Updated scope for milestone {}.\n",
                escape(&milestone_id)
            );
            push_scope_counts(&mut output, &counts);
            push_field(
                &mut output,
                "Accepted review",
                if review_removed {
                    "removed"
                } else {
                    "unchanged"
                },
            );
            CommandOutput::success(output.into_bytes())
        }
        Ok(milestone::ScopeUpdateOutcome::NoChange { milestone_id }) => CommandOutput::no_change(
            "MILESTONE_SCOPE_UNCHANGED",
            &format!(
                "Milestone {} already has the submitted scope.",
                escape(&milestone_id)
            ),
        ),
        Err(error) => render_milestone_failure(
            "MILESTONE_SCOPE_UPDATE_FAILED",
            "Cannot update the milestone scope.",
            &error,
        ),
    }
}

#[must_use]
pub fn milestone_rebaseline(start: &Path, revision: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match milestone::rebaseline(&paths.project_root, &paths.specbind_root, revision) {
        Ok(milestone::RebaselineOutcome::Rebaselined {
            milestone_id,
            previous,
            baseline_revision,
            review_removed,
        }) => {
            let mut output = format!(
                "OK MILESTONE_REBASELINED: Rebaselined milestone {}.\n",
                escape(&milestone_id)
            );
            push_field(&mut output, "Previous baseline", &previous);
            push_field(&mut output, "Baseline revision", &baseline_revision);
            push_field(
                &mut output,
                "Accepted review",
                if review_removed {
                    "removed"
                } else {
                    "unchanged"
                },
            );
            CommandOutput::success(output.into_bytes())
        }
        Ok(milestone::RebaselineOutcome::NoChange {
            milestone_id,
            baseline_revision,
        }) => CommandOutput::no_change(
            "MILESTONE_BASELINE_UNCHANGED",
            &format!(
                "Milestone {} is already based on {}.",
                escape(&milestone_id),
                escape(&baseline_revision)
            ),
        ),
        Err(error) => render_milestone_failure(
            "MILESTONE_REBASELINE_FAILED",
            "Cannot rebaseline the active milestone.",
            &error,
        ),
    }
}

fn push_scope_counts(output: &mut String, counts: &milestone::ScopeCounts) {
    push_field(output, "New specs", &counts.new_specs.to_string());
    push_field(output, "Spec updates", &counts.spec_updates.to_string());
    push_field(output, "Direct changes", &counts.direct_changes.to_string());
}

fn read_scope_document(
    start: &Path,
    project_root: &Path,
    source: &str,
) -> Result<String, ExternalInputError> {
    read_external_input(&SCOPE_INPUT, start, project_root, source)
}

fn render_milestone_failure(
    code: &'static str,
    message: &str,
    error: &milestone::MilestoneIssues,
) -> CommandOutput {
    CommandOutput::failure(
        code,
        message,
        error.issues.iter().map(render_milestone_issue).collect(),
    )
}

fn render_milestone_mutation_failure(
    message: &str,
    error: milestone::MilestoneIssues,
) -> CommandOutput {
    let mut issues = error.issues.into_iter();
    let Some(first) = issues.next() else {
        return CommandOutput::failure("MILESTONE_MUTATION_FAILED", message, vec![]);
    };
    let mut details = vec![render_milestone_issue(&first)];
    details.extend(issues.map(|issue| render_milestone_issue(&issue)));
    CommandOutput::failure(first.code, message, details)
}

fn render_milestone_issue(issue: &MilestoneIssue) -> String {
    let path = issue
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path)));
    format!("{}{path} {}", issue.code, escape(&issue.message))
}

#[must_use]
pub fn release_preflight(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match release_readiness::resolve(&paths.project_root, &paths.specbind_root) {
        Ok(readiness) => {
            let mut output = format!(
                "OK RELEASE_READY: Release {} is ready for project release work across {} specs.\n  Milestone ID: {}\n  Specs: {}\n  Direct changes: {}\n  Mutation targets:\n",
                escape(&readiness.version),
                readiness.specs.len(),
                escape(&readiness.milestone_id),
                if readiness.specs.is_empty() {
                    "none".to_owned()
                } else {
                    readiness
                        .specs
                        .iter()
                        .map(|spec| escape(spec))
                        .collect::<Vec<_>>()
                        .join(", ")
                },
                readiness.direct_changes,
            );
            for target in readiness.mutation_targets {
                let state = match target.state {
                    MutationTargetState::Existing => "existing",
                    MutationTargetState::Absent => "absent",
                };
                writeln!(output, "    - {state} {}", escape(&target.path))
                    .expect("write to String");
            }
            CommandOutput::success(output.into_bytes())
        }
        Err(error) => CommandOutput::failure(
            error.code,
            "Release preflight failed.",
            error
                .diagnostics
                .iter()
                .map(render_release_diagnostic)
                .collect(),
        ),
    }
}

#[must_use]
pub fn release_finalize(start: &Path, log_entries_source: Option<&str>) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let log_entries = match log_entries_source {
        Some(source) => match read_release_log_entries(start, &paths.project_root, source) {
            Ok(input) => Some(input),
            Err(output) => return output,
        },
        None => None,
    };
    match release_finalize::finalize(
        &paths.project_root,
        &paths.specbind_root,
        paths.language,
        log_entries.as_deref(),
    ) {
        Ok(release_finalize::FinalizeOutcome::Finalized { version, specs }) => {
            CommandOutput::success(
                format!(
                    "OK RELEASE_FINALIZED: Finalized {} for {} specs.\n",
                    escape(&version),
                    specs
                )
                .into_bytes(),
            )
        }
        Ok(release_finalize::FinalizeOutcome::AlreadyFinalized { version, .. }) => {
            CommandOutput::no_change(
                "RELEASE_ALREADY_FINALIZED",
                &format!("Release {} is already finalized.", escape(&version)),
            )
        }
        Err(error) => {
            let mut issues = error.issues.into_iter();
            let Some(first) = issues.next() else {
                return CommandOutput::failure(
                    "RELEASE_FINALIZE_FAILED",
                    "Release finalization failed.",
                    vec![],
                );
            };
            let mut details = vec![render_finalize_issue(&first)];
            details.extend(issues.map(|issue| render_finalize_issue(&issue)));
            CommandOutput::failure(first.code, "Release finalization failed.", details)
        }
    }
}

fn read_release_log_entries(
    start: &Path,
    project_root: &Path,
    source: &str,
) -> Result<String, CommandOutput> {
    read_external_input(&LOG_ENTRIES_INPUT, start, project_root, source)
        .map_err(|error| CommandOutput::failure(error.code, error.message, vec![]))
}

fn render_finalize_issue(issue: &FinalizeIssue) -> String {
    let path = issue
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path)));
    format!("{}{path} {}", issue.code, escape(&issue.message))
}

fn render_release_diagnostic(diagnostic: &ReleaseDiagnostic) -> String {
    let path = diagnostic
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path)));
    format!("{}{path} {}", diagnostic.code, escape(&diagnostic.message))
}

#[must_use]
pub fn spec_gate_approve(
    start: &Path,
    canonical_spec: &str,
    gate: approval::Gate,
    approval_mode: &str,
    delegation_workflow: Option<&str>,
    requirement_ids: Option<&str>,
) -> CommandOutput {
    let failure_code = approve_failed_code(gate);
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let mode = match (approval_mode, delegation_workflow) {
        ("explicit", None) => approval::ApprovalMode::Explicit,
        ("explicit", Some(_)) => {
            return CommandOutput::failure(
                failure_code,
                approve_failure_message(gate, canonical_spec),
                vec![
                    "SPEC_GATE_DELEGATION_INVALID explicit approval does not accept a delegation workflow"
                        .to_owned(),
                ],
            );
        }
        ("delegated", Some(workflow)) => approval::ApprovalMode::Delegated {
            workflow: workflow.to_owned(),
        },
        ("delegated", None) => {
            return CommandOutput::failure(
                failure_code,
                approve_failure_message(gate, canonical_spec),
                vec![
                    "SPEC_GATE_DELEGATION_INVALID delegated approval requires --delegation-workflow"
                        .to_owned(),
                ],
            );
        }
        _ => {
            return CommandOutput::failure(
                failure_code,
                approve_failure_message(gate, canonical_spec),
                vec![
                    "SPEC_GATE_APPROVAL_MODE_INVALID approval mode must be explicit or delegated"
                        .to_owned(),
                ],
            );
        }
    };
    let request = approval::ApprovalRequest {
        gate,
        mode,
        requirement_ids: requirement_ids
            .map(|value| {
                value
                    .split(',')
                    .map(str::trim)
                    .filter(|id| !id.is_empty())
                    .map(ToOwned::to_owned)
                    .collect()
            })
            .unwrap_or_default(),
    };
    match approval::approve(
        &paths.project_root,
        &paths.specbind_root,
        canonical_spec,
        &request,
    ) {
        Ok(approval::ApproveOutcome::Approved(approved)) => CommandOutput::success(
            render_gate_approval(gate, canonical_spec, &approved).into_bytes(),
        ),
        Ok(approval::ApproveOutcome::AlreadyApproved(_)) => CommandOutput::no_change(
            already_approved_code(gate),
            &format!(
                "Spec {} already has identical fresh {} approval.",
                escape(canonical_spec),
                gate.name()
            ),
        ),
        Err(error) => CommandOutput::failure(
            failure_code,
            approve_failure_message(gate, canonical_spec),
            error.issues.iter().map(render_approval_issue).collect(),
        ),
    }
}

#[must_use]
pub fn spec_gate_invalidate(
    start: &Path,
    canonical_spec: &str,
    gate: approval::Gate,
) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match approval::invalidate(
        &paths.project_root,
        &paths.specbind_root,
        canonical_spec,
        gate,
    ) {
        Ok(approval::InvalidateOutcome::Invalidated { state }) => CommandOutput::success(
            format!(
                "OK SPEC_{}_INVALIDATED: Invalidated {} for spec {}.\n  State: {}\n",
                gate.name().to_uppercase(),
                gate.name(),
                escape(canonical_spec),
                approval::state_name(state)
            )
            .into_bytes(),
        ),
        Ok(approval::InvalidateOutcome::NoChange) => CommandOutput::no_change(
            not_approved_code(gate),
            &format!(
                "Spec {} has no {} approval to invalidate.",
                escape(canonical_spec),
                gate.name()
            ),
        ),
        Err(error) => CommandOutput::failure(
            invalidate_failed_code(gate),
            format!(
                "Cannot invalidate {} for spec {canonical_spec}.",
                gate.name()
            ),
            error.issues.iter().map(render_approval_issue).collect(),
        ),
    }
}

fn render_gate_approval(
    gate: approval::Gate,
    canonical_spec: &str,
    approved: &approval::GateApproval,
) -> String {
    let mut output = format!(
        "OK SPEC_{}_APPROVED: Approved {} for spec {}.\n",
        gate.name().to_uppercase(),
        gate.name(),
        escape(canonical_spec)
    );
    push_field(&mut output, "State", approval::state_name(approved.state));
    push_field(&mut output, "Approval mode", approved.approval_mode);
    if let Some(workflow) = &approved.delegation_workflow {
        push_field(&mut output, "Delegation workflow", workflow);
    }
    push_field(&mut output, "Passed at", &approved.passed_at);
    if let Some(count) = approved.approved_requirement_ids {
        push_field(&mut output, "Approved requirement IDs", &count.to_string());
    }
    output
}

fn approve_failed_code(gate: approval::Gate) -> &'static str {
    match gate {
        approval::Gate::Requirements => "SPEC_REQUIREMENTS_APPROVE_FAILED",
        approval::Gate::Design => "SPEC_DESIGN_APPROVE_FAILED",
        approval::Gate::Tasks => "SPEC_TASKS_APPROVE_FAILED",
    }
}

fn invalidate_failed_code(gate: approval::Gate) -> &'static str {
    match gate {
        approval::Gate::Requirements => "SPEC_REQUIREMENTS_INVALIDATE_FAILED",
        approval::Gate::Design => "SPEC_DESIGN_INVALIDATE_FAILED",
        approval::Gate::Tasks => "SPEC_TASKS_INVALIDATE_FAILED",
    }
}

fn already_approved_code(gate: approval::Gate) -> &'static str {
    match gate {
        approval::Gate::Requirements => "SPEC_REQUIREMENTS_ALREADY_APPROVED",
        approval::Gate::Design => "SPEC_DESIGN_ALREADY_APPROVED",
        approval::Gate::Tasks => "SPEC_TASKS_ALREADY_APPROVED",
    }
}

fn not_approved_code(gate: approval::Gate) -> &'static str {
    match gate {
        approval::Gate::Requirements => "SPEC_REQUIREMENTS_NOT_APPROVED",
        approval::Gate::Design => "SPEC_DESIGN_NOT_APPROVED",
        approval::Gate::Tasks => "SPEC_TASKS_NOT_APPROVED",
    }
}

fn approve_failure_message(gate: approval::Gate, canonical_spec: &str) -> String {
    format!("Cannot approve {} for spec {canonical_spec}.", gate.name())
}

fn render_approval_issue(issue: &approval::ApprovalIssue) -> String {
    let path = issue
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path)));
    format!("{}{path} {}", issue.code, escape(&issue.message))
}

#[must_use]
pub fn milestone_review_status(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let report = cross_spec_review::evaluate_freshness(&paths.project_root, &paths.specbind_root);
    let details = report.issues.iter().map(render_review_issue).collect();
    let reportable = matches!(
        report.status,
        ReviewFreshnessStatus::NotRequired
            | ReviewFreshnessStatus::Missing
            | ReviewFreshnessStatus::Fresh
            | ReviewFreshnessStatus::Stale
    );
    let Some(milestone_id) = report.milestone_id.as_deref().filter(|_| reportable) else {
        return CommandOutput::failure(
            "MILESTONE_REVIEW_STATUS_FAILED",
            "Cannot report the cross-spec review status.",
            details,
        );
    };
    let mut output = format!(
        "OK MILESTONE_REVIEW_STATUS_REPORTED: Reported cross-spec review status for milestone {}.\n",
        escape(milestone_id)
    );
    push_field(
        &mut output,
        "Status",
        milestone_status::review_name(report.status),
    );
    if let Some(accepted) = &report.accepted {
        push_field(&mut output, "Passed at", &accepted.passed_at);
        push_field(
            &mut output,
            "Inputs",
            &accepted.input_revisions.len().to_string(),
        );
    }
    if !details.is_empty() {
        output.push_str("  Diagnostics:\n");
        for detail in details {
            writeln!(output, "    - {detail}").expect("writing to a String cannot fail");
        }
    }
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn milestone_review_accept(start: &Path, candidate_source: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let candidate = match read_external_input(
        &REVIEW_CANDIDATE_INPUT,
        start,
        &paths.project_root,
        candidate_source,
    ) {
        Ok(candidate) => candidate,
        Err(error) => {
            return CommandOutput::failure(
                "MILESTONE_REVIEW_ACCEPT_FAILED",
                "Cannot accept the cross-spec review.",
                vec![format!("{} {}", error.code, escape(&error.message))],
            );
        }
    };
    match cross_spec_review::accept(&paths.project_root, &paths.specbind_root, &candidate) {
        Ok(accepted) => CommandOutput::success(
            format!(
                "OK MILESTONE_REVIEW_ACCEPTED: Accepted cross-spec review for milestone {}.\n  Passed at: {}\n  Inputs: {}\n",
                escape(&accepted.milestone_id),
                escape(&accepted.passed_at),
                accepted.input_revisions.len()
            )
            .into_bytes(),
        ),
        Err(error) => CommandOutput::failure(
            "MILESTONE_REVIEW_ACCEPT_FAILED",
            "Cannot accept the cross-spec review.",
            error.issues.iter().map(render_review_issue).collect(),
        ),
    }
}

fn render_review_issue(issue: &ReviewIssue) -> String {
    let path = issue
        .source
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path)));
    format!("{}{path} {}", issue.code, escape(&issue.message))
}

#[must_use]
pub fn milestone_status(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match milestone_status::resolve(&paths.project_root, &paths.specbind_root) {
        Ok(Some(model)) => render_milestone_status(&model),
        Ok(None) => CommandOutput::no_change("NO_ACTIVE_MILESTONE", "No active milestone exists."),
        Err(error) => CommandOutput::failure(
            "MILESTONE_STATUS_FAILED",
            "Cannot report the active milestone.",
            error
                .diagnostics
                .iter()
                .map(render_milestone_diagnostic)
                .collect(),
        ),
    }
}

fn render_milestone_status(model: &MilestoneStatusModel) -> CommandOutput {
    let mut output = format!(
        "OK MILESTONE_STATUS_REPORTED: Reported the active milestone.\n  Milestone: {}\n",
        escape(&model.milestone_id)
    );
    push_field(
        &mut output,
        "Target release",
        model.target_release.as_deref().unwrap_or("none"),
    );
    push_field(
        &mut output,
        "Stage",
        milestone_status::stage_name(model.stage),
    );
    push_field(
        &mut output,
        "Health",
        match model.health {
            MilestoneHealth::Consistent => "consistent",
            MilestoneHealth::Inconsistent => "inconsistent",
        },
    );
    push_field(
        &mut output,
        "Cross-spec review",
        milestone_status::review_name(model.review_status),
    );
    let spec_counts = if model.spec_state_counts.is_empty() {
        "none".to_owned()
    } else {
        model
            .spec_state_counts
            .iter()
            .map(|(state, count)| format!("{state}={count}"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    push_field(&mut output, "Spec states", &spec_counts);
    push_field(
        &mut output,
        "Direct progress",
        &format!(
            "{}/{} completed",
            model.direct_completed, model.direct_total
        ),
    );
    push_field(
        &mut output,
        "Revision",
        model.current_revision.as_deref().unwrap_or("unavailable"),
    );
    render_milestone_items(model, &mut output);
    render_milestone_actions(model, &mut output);
    push_inline_list(&mut output, "Release blockers", &model.release_blockers);
    render_milestone_diagnostics(model, &mut output);
    CommandOutput::success(output.into_bytes())
}

fn render_milestone_items(model: &MilestoneStatusModel, output: &mut String) {
    output.push_str("  Items:\n");
    for item in &model.items {
        let waiting = if item.waiting_for.is_empty() {
            String::new()
        } else {
            format!(" waiting_for={}", item.waiting_for.join(","))
        };
        writeln!(
            output,
            "    - {} status={}{} summary={}",
            escape(&item.id),
            escape(&item.status),
            escape(&waiting),
            escape(&item.summary)
        )
        .expect("writing to a String cannot fail");
    }
}

fn render_milestone_actions(model: &MilestoneStatusModel, output: &mut String) {
    if model.actionable.is_empty() {
        push_field(output, "Actionable", "none");
        return;
    }
    output.push_str("  Actionable:\n");
    for action in &model.actionable {
        writeln!(
            output,
            "    - {} action={}",
            escape(&action.item),
            action.action
        )
        .expect("writing to a String cannot fail");
    }
}

fn render_milestone_diagnostics(model: &MilestoneStatusModel, output: &mut String) {
    if model.diagnostics.is_empty() {
        push_field(output, "Diagnostics", "none");
        return;
    }
    output.push_str("  Diagnostics:\n");
    for diagnostic in &model.diagnostics {
        writeln!(output, "    - {}", render_milestone_diagnostic(diagnostic))
            .expect("writing to a String cannot fail");
    }
}

fn render_milestone_diagnostic(diagnostic: &milestone_status::MilestoneDiagnostic) -> String {
    let path = diagnostic
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path)));
    format!("{}{path} {}", diagnostic.code, escape(&diagnostic.message))
}

fn render_spec_status(canonical_spec: &str, model: &SpecStatusModel) -> CommandOutput {
    let mut output = format!(
        "OK SPEC_STATUS_REPORTED: Reported status for spec {}.\n",
        escape(canonical_spec)
    );
    push_field(
        &mut output,
        "State",
        spec_status::state_name(model.declared_state),
    );
    push_field(
        &mut output,
        "Milestone",
        model.milestone_id.as_deref().unwrap_or("none"),
    );
    push_field(
        &mut output,
        "Health",
        match model.health {
            ConsistencyHealth::Consistent => "consistent",
            ConsistencyHealth::Inconsistent => "inconsistent",
        },
    );
    push_field(
        &mut output,
        "Gates",
        &format!(
            "requirements={}, design={}, tasks={}, completion={}",
            spec_status::freshness_name(model.freshness.requirements.status),
            spec_status::freshness_name(model.freshness.design.status),
            spec_status::freshness_name(model.freshness.tasks.status),
            spec_status::freshness_name(model.freshness.completion.status),
        ),
    );
    render_status_tasks(model, &mut output);
    render_status_coverage(model, &mut output);
    render_status_diagnostics(model, &mut output);
    CommandOutput::success(output.into_bytes())
}

fn render_status_tasks(model: &SpecStatusModel, output: &mut String) {
    if let Some(tasks) = &model.task_model {
        push_field(
            output,
            "Task progress",
            &format!(
                "{} total, {} completed, {} pending, {} blocked",
                tasks.total(),
                tasks.completed,
                tasks.pending,
                tasks.blocked
            ),
        );
        push_inline_list(output, "Next actionable", &tasks.actionable_ids);
    } else {
        push_field(output, "Task progress", "unavailable");
        push_field(output, "Next actionable", "none");
    }
    if model.blockers.is_empty() {
        push_field(output, "Blockers", "none");
    } else {
        output.push_str("  Blockers:\n");
        for blocker in &model.blockers {
            writeln!(
                output,
                "    - {}: {}",
                escape(&blocker.task_id),
                escape(&blocker.reason)
            )
            .expect("writing to a String cannot fail");
        }
    }
}

fn render_status_coverage(model: &SpecStatusModel, output: &mut String) {
    if let Some(coverage) = &model.coverage {
        push_field(
            output,
            "Requirement coverage",
            &format!(
                "design {}/{}, tasks {}/{}{}",
                coverage.design,
                coverage.active,
                coverage.tasks,
                coverage.active,
                if coverage.tasks_required {
                    " (required)"
                } else {
                    " (not required)"
                }
            ),
        );
    } else {
        push_field(output, "Requirement coverage", "inactive");
    }
}

fn render_status_diagnostics(model: &SpecStatusModel, output: &mut String) {
    if model.diagnostics.is_empty() {
        push_field(output, "Diagnostics", "none");
    } else {
        output.push_str("  Diagnostics:\n");
        for diagnostic in &model.diagnostics {
            let path = diagnostic
                .path
                .as_ref()
                .map_or_else(String::new, |path| format!(" {}:", escape(path)));
            writeln!(
                output,
                "    - {}{path} {}",
                diagnostic.code,
                escape(&diagnostic.message)
            )
            .expect("writing to a String cannot fail");
        }
    }
}

fn load_task_model(start: &Path, canonical_spec: &str) -> Result<TaskReadModel, CommandOutput> {
    let paths = config::resolve_from(start)
        .map_err(|error| CommandOutput::failure(error.code, error.message, vec![]))?;
    let resolution = artifacts::resolve_tasks(&paths.specbind_root, canonical_spec);
    match resolution.tasks {
        Some(tasks) if resolution.issues.is_empty() => Ok(TaskReadModel::derive(&tasks)),
        _ => Err(CommandOutput::failure(
            "TASKS_READ_FAILED",
            format!("Cannot derive tasks for spec {canonical_spec}."),
            resolution.issues.iter().map(render_issue).collect(),
        )),
    }
}

fn render_group(group: &GroupView, output: &mut String) {
    let total = group.tasks.len();
    let status = if group.completed == total {
        "completed"
    } else if group.completed == 0 {
        "pending"
    } else {
        "partial"
    };
    writeln!(
        output,
        "  [{status} {}/{}; {} blocked] {} {}",
        group.completed,
        total,
        group.blocked,
        escape(&group.id),
        escape(&group.title)
    )
    .expect("writing to a String cannot fail");
    for task in &group.tasks {
        output.push_str("    ");
        output.push_str(&render_task_summary(task));
        output.push('\n');
    }
}

fn render_task_summary(task: &TaskView) -> String {
    format!(
        "[{}] {} {}",
        render_status(task),
        escape(&task.id),
        escape(&task.title)
    )
}

fn render_status(task: &TaskView) -> String {
    match task.status {
        TaskStatus::Completed => "completed".to_owned(),
        TaskStatus::Blocked => "blocked".to_owned(),
        TaskStatus::Pending if task.actionable => "pending actionable".to_owned(),
        TaskStatus::Pending => "pending waiting".to_owned(),
    }
}

fn push_field(output: &mut String, label: &str, value: &str) {
    output.push_str("  ");
    output.push_str(label);
    output.push_str(": ");
    output.push_str(&escape(value));
    output.push('\n');
}

fn push_inline_list(output: &mut String, label: &str, values: &[String]) {
    let value = if values.is_empty() {
        "none".to_owned()
    } else {
        values
            .iter()
            .map(|value| escape(value))
            .collect::<Vec<_>>()
            .join(", ")
    };
    push_field(output, label, &value);
}

fn push_list(output: &mut String, label: &str, values: &[String]) {
    if values.is_empty() {
        push_field(output, label, "none");
        return;
    }
    output.push_str("  ");
    output.push_str(label);
    output.push_str(":\n");
    for value in values {
        output.push_str("    - ");
        output.push_str(&escape(value));
        output.push('\n');
    }
}

fn render_artifact(artifact: &Artifact) -> String {
    let mut output = format!(
        "selector={} type=\"{}\" path={}",
        escape(&artifact.selector),
        escape(&artifact.artifact_type),
        escape(artifact.path.as_str())
    );
    if let Some(artifact_id) = &artifact.artifact_id {
        output.push_str(" artifact_id=");
        output.push_str(&escape(artifact_id));
    }
    output
}

fn render_issue(issue: &DiscoveryIssue) -> String {
    let path = issue
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path.as_str())));
    format!("{}{path} {}", issue.code, escape(&issue.message))
}

fn escape(value: &str) -> String {
    value
        .chars()
        .flat_map(|value| match value {
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect(),
            '\t' => "\\t".chars().collect(),
            value if value.is_control() || value == '\u{1b}' => {
                format!("\\u{{{:x}}}", u32::from(value)).chars().collect()
            }
            value => vec![value],
        })
        .collect()
}
