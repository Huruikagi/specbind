//! Spec and milestone status command execution and rendering.

use super::super::*;
use serde::Serialize;

#[derive(Serialize)]
struct JsonResponse<T> {
    status: &'static str,
    code: &'static str,
    data: T,
}

#[derive(Serialize)]
struct JsonFailure {
    status: &'static str,
    code: String,
    message: String,
    details: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SpecStatusData<'a> {
    spec: &'a str,
    state: &'static str,
    milestone: Option<&'a str>,
    health: &'static str,
    gates: GateStatusData,
    next_action: &'static str,
    expected_requirements_work: bool,
    expected_design_work: Option<ExpectedDesignWorkData>,
    contract_review: Option<&'static str>,
    delegated_gates: Option<Vec<DelegatedGateData<'a>>>,
    tasks: Option<TaskStatusData<'a>>,
    coverage: Option<RequirementCoverageData>,
    diagnostics: Vec<StatusDiagnosticData<'a>>,
}

#[derive(Serialize)]
struct GateStatusData {
    requirements: &'static str,
    design: &'static str,
    tasks: &'static str,
    completion: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExpectedDesignWorkData {
    missing_coverage: usize,
}

#[derive(Serialize)]
struct DelegatedGateData<'a> {
    gate: &'static str,
    workflow: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TaskStatusData<'a> {
    total: usize,
    completed: usize,
    pending: usize,
    blocked: usize,
    next_tasks: &'a [String],
    blockers: Vec<TaskBlockerData<'a>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TaskBlockerData<'a> {
    task_id: &'a str,
    reason: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RequirementCoverageData {
    active: usize,
    design: usize,
    tasks: usize,
    tasks_required: bool,
}

#[derive(Serialize)]
struct StatusDiagnosticData<'a> {
    code: &'static str,
    path: Option<&'a str>,
    message: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MilestoneStatusData<'a> {
    milestone_id: &'a str,
    target_release: Option<&'a str>,
    stage: &'static str,
    health: &'static str,
    contract_review: &'static str,
    spec_states: &'a std::collections::BTreeMap<String, usize>,
    direct_progress: DirectProgressData,
    revision: Option<&'a str>,
    baseline: &'a str,
    items: Vec<MilestoneItemData<'a>>,
    actionable: Vec<MilestoneActionData<'a>>,
    current_blockers: &'a [String],
    release_readiness_evaluated: bool,
    release_blockers: Option<&'a [String]>,
    diagnostics: Vec<StatusDiagnosticData<'a>>,
}

#[derive(Serialize)]
struct DirectProgressData {
    completed: usize,
    total: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MilestoneItemData<'a> {
    id: &'a str,
    summary: &'a str,
    status: &'a str,
    waiting_for: &'a [String],
}

#[derive(Serialize)]
struct MilestoneActionData<'a> {
    item: &'a str,
    action: &'static str,
}

#[must_use]
pub fn spec_status(start: &Path, canonical_spec: &str, json: bool) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) if json => return render_json_failure(error.code, error.message, vec![]),
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let model =
        match spec_status::resolve(&paths.project_root, &paths.specbind_root, canonical_spec) {
            Ok(model) => model,
            Err(error) => {
                let details = error.issues.iter().map(render_issue).collect();
                if json {
                    return render_json_failure(
                        "SPEC_STATUS_FAILED",
                        format!("Cannot report status for spec {canonical_spec}."),
                        details,
                    );
                }
                return CommandOutput::failure(
                    "SPEC_STATUS_FAILED",
                    format!("Cannot report status for spec {canonical_spec}."),
                    details,
                );
            }
        };
    if json {
        render_spec_status_json(canonical_spec, &model)
    } else {
        render_spec_status(canonical_spec, &model)
    }
}

#[must_use]
pub fn milestone_status(start: &Path, json: bool) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) if json => return render_json_failure(error.code, error.message, vec![]),
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match milestone_status::resolve(&paths.project_root, &paths.specbind_root) {
        Ok(Some(model)) if json => render_milestone_status_json(&model),
        Ok(Some(model)) => render_milestone_status(&model),
        Ok(None) if json => render_json(
            &JsonResponse {
                status: "no_change",
                code: "NO_ACTIVE_MILESTONE",
                data: Option::<()>::None,
            },
            true,
        ),
        Ok(None) => CommandOutput::no_change("NO_ACTIVE_MILESTONE", "No active milestone exists."),
        Err(error) if json => render_json_failure(
            "MILESTONE_STATUS_FAILED",
            "Cannot report the active milestone.",
            error
                .diagnostics
                .iter()
                .map(render_milestone_diagnostic)
                .collect(),
        ),
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
        "Contract review",
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
    push_field(&mut output, "Baseline", &model.baseline_revision);
    render_milestone_items(model, &mut output);
    render_milestone_actions(model, &mut output);
    if !model.current_blockers.is_empty() {
        push_inline_list(&mut output, "Current blockers", &model.current_blockers);
        if model
            .current_blockers
            .iter()
            .any(|blocker| blocker == "WORKTREE_NOT_CLEAN")
        {
            push_field(
                &mut output,
                "Worktree action",
                "review and commit or otherwise reconcile current changes to continue",
            );
        }
    }
    if release_readiness_evaluated(model.stage) {
        push_inline_list(&mut output, "Release blockers", &model.release_blockers);
    } else {
        push_field(
            &mut output,
            "Release readiness",
            "not evaluated until validation",
        );
    }
    render_milestone_diagnostics(model, &mut output);
    CommandOutput::success(output.into_bytes())
}

fn render_milestone_status_json(model: &MilestoneStatusModel) -> CommandOutput {
    let release_readiness_evaluated = release_readiness_evaluated(model.stage);
    let data = MilestoneStatusData {
        milestone_id: &model.milestone_id,
        target_release: model.target_release.as_deref(),
        stage: milestone_status::stage_name(model.stage),
        health: match model.health {
            MilestoneHealth::Consistent => "consistent",
            MilestoneHealth::Inconsistent => "inconsistent",
        },
        contract_review: milestone_status::review_name(model.review_status),
        spec_states: &model.spec_state_counts,
        direct_progress: DirectProgressData {
            completed: model.direct_completed,
            total: model.direct_total,
        },
        revision: model.current_revision.as_deref(),
        baseline: &model.baseline_revision,
        items: model
            .items
            .iter()
            .map(|item| MilestoneItemData {
                id: &item.id,
                summary: &item.summary,
                status: &item.status,
                waiting_for: &item.waiting_for,
            })
            .collect(),
        actionable: model
            .actionable
            .iter()
            .map(|action| MilestoneActionData {
                item: &action.item,
                action: action.action,
            })
            .collect(),
        current_blockers: &model.current_blockers,
        release_readiness_evaluated,
        release_blockers: release_readiness_evaluated.then_some(model.release_blockers.as_slice()),
        diagnostics: model
            .diagnostics
            .iter()
            .map(|diagnostic| StatusDiagnosticData {
                code: diagnostic.code,
                path: diagnostic.path.as_deref(),
                message: &diagnostic.message,
            })
            .collect(),
    };
    render_json(
        &JsonResponse {
            status: "ok",
            code: "MILESTONE_STATUS_REPORTED",
            data,
        },
        true,
    )
}

fn release_readiness_evaluated(stage: milestone_status::DeliveryStage) -> bool {
    matches!(
        stage,
        milestone_status::DeliveryStage::Validation
            | milestone_status::DeliveryStage::ReleasePending
            | milestone_status::DeliveryStage::ReleaseReady
    )
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
    push_field(
        &mut output,
        "Next action",
        spec_status::action_name(model.next_action),
    );
    if model.expected_requirements_work {
        push_field(&mut output, "Expected work", "author Requirements");
    } else if let Some(expected) = model.expected_design_work {
        push_field(
            &mut output,
            "Expected work",
            &format!(
                "cover {} active requirement(s) in Design",
                expected.missing_coverage
            ),
        );
    }
    if let Some(review) = model.contract_review {
        push_field(
            &mut output,
            "Contract review",
            milestone_status::review_name(review),
        );
    }
    if let Some(delegated) = &model.delegated_gates {
        push_field(
            &mut output,
            "Delegated gates",
            &if delegated.is_empty() {
                "none".to_owned()
            } else {
                delegated
                    .iter()
                    .map(|gate| format!("{} ({})", gate.gate, escape(&gate.workflow)))
                    .collect::<Vec<_>>()
                    .join(", ")
            },
        );
    }
    render_status_tasks(model, &mut output);
    render_status_coverage(model, &mut output);
    render_status_diagnostics(model, &mut output);
    CommandOutput::success(output.into_bytes())
}

fn render_spec_status_json(canonical_spec: &str, model: &SpecStatusModel) -> CommandOutput {
    let data = SpecStatusData {
        spec: canonical_spec,
        state: spec_status::state_name(model.declared_state),
        milestone: model.milestone_id.as_deref(),
        health: match model.health {
            ConsistencyHealth::Consistent => "consistent",
            ConsistencyHealth::Inconsistent => "inconsistent",
        },
        gates: GateStatusData {
            requirements: spec_status::freshness_name(model.freshness.requirements.status),
            design: spec_status::freshness_name(model.freshness.design.status),
            tasks: spec_status::freshness_name(model.freshness.tasks.status),
            completion: spec_status::freshness_name(model.freshness.completion.status),
        },
        next_action: spec_status::action_name(model.next_action),
        expected_requirements_work: model.expected_requirements_work,
        expected_design_work: model
            .expected_design_work
            .map(|work| ExpectedDesignWorkData {
                missing_coverage: work.missing_coverage,
            }),
        contract_review: model.contract_review.map(milestone_status::review_name),
        delegated_gates: model.delegated_gates.as_ref().map(|gates| {
            gates
                .iter()
                .map(|gate| DelegatedGateData {
                    gate: gate.gate,
                    workflow: &gate.workflow,
                })
                .collect()
        }),
        tasks: model.task_model.as_ref().map(|tasks| TaskStatusData {
            total: tasks.total(),
            completed: tasks.completed,
            pending: tasks.pending,
            blocked: tasks.blocked,
            next_tasks: &tasks.actionable_ids,
            blockers: model
                .blockers
                .iter()
                .map(|blocker| TaskBlockerData {
                    task_id: &blocker.task_id,
                    reason: &blocker.reason,
                })
                .collect(),
        }),
        coverage: model
            .coverage
            .as_ref()
            .map(|coverage| RequirementCoverageData {
                active: coverage.active,
                design: coverage.design,
                tasks: coverage.tasks,
                tasks_required: coverage.tasks_required,
            }),
        diagnostics: model
            .diagnostics
            .iter()
            .map(|diagnostic| StatusDiagnosticData {
                code: diagnostic.code,
                path: diagnostic.path.as_deref(),
                message: &diagnostic.message,
            })
            .collect(),
    };
    render_json(
        &JsonResponse {
            status: "ok",
            code: "SPEC_STATUS_REPORTED",
            data,
        },
        true,
    )
}

fn render_json_failure(
    code: impl Into<String>,
    message: impl Into<String>,
    details: Vec<String>,
) -> CommandOutput {
    render_json(
        &JsonFailure {
            status: "error",
            code: code.into(),
            message: message.into(),
            details,
        },
        false,
    )
}

fn render_json(value: &impl Serialize, success: bool) -> CommandOutput {
    let mut stdout = serde_json::to_vec(value).expect("spec status JSON response is serializable");
    stdout.push(b'\n');
    CommandOutput {
        stdout,
        stderr: vec![],
        success,
    }
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
        push_inline_list(output, "Next task", &tasks.actionable_ids);
    } else {
        push_field(output, "Task progress", "unavailable");
        push_field(output, "Next task", "none");
    }
    if model.blockers.is_empty() {
        push_field(output, "Task blockers", "none");
    } else {
        output.push_str("  Task blockers:\n");
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
