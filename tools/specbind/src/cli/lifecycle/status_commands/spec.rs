use super::super::super::*;
use super::json::{self, JsonResponse};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SpecStatusData<'a> {
    spec: &'a str,
    state: &'static str,
    milestone: Option<&'a str>,
    health: &'static str,
    semantic_alignment: &'static str,
    gates: GateStatusData,
    next_action: &'static str,
    expected_requirements_work: bool,
    expected_design_work: Option<ExpectedDesignWorkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_plan_authority: Option<TaskPlanAuthorityData>,
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
#[serde(rename_all = "camelCase")]
struct TaskPlanAuthorityData {
    status: &'static str,
    next_action: &'static str,
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

#[must_use]
pub fn spec_status(start: &Path, canonical_spec: &str, json_output: bool) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) if json_output => {
            return json::render_failure(error.code, error.message, vec![]);
        }
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let model =
        match spec_status::resolve(&paths.project_root, &paths.specbind_root, canonical_spec) {
            Ok(model) => model,
            Err(error) => {
                let details = error.issues.iter().map(render_issue).collect();
                if json_output {
                    return json::render_failure(
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
    if json_output {
        render_json(canonical_spec, &model)
    } else {
        render_text(canonical_spec, &model)
    }
}

fn render_text(canonical_spec: &str, model: &SpecStatusModel) -> CommandOutput {
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
        "State health",
        match model.health {
            ConsistencyHealth::Consistent => "consistent",
            ConsistencyHealth::Inconsistent => "inconsistent",
        },
    );
    push_field(&mut output, "Semantic alignment", "not evaluated");
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
    if model.task_plan_authority.is_some() {
        push_field(
            &mut output,
            "Task plan authority",
            "not current; reconcile in Tasks phase",
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
    render_tasks(model, &mut output);
    render_coverage(model, &mut output);
    render_diagnostics(model, &mut output);
    CommandOutput::success(output.into_bytes())
}

fn render_json(canonical_spec: &str, model: &SpecStatusModel) -> CommandOutput {
    let data = SpecStatusData {
        spec: canonical_spec,
        state: spec_status::state_name(model.declared_state),
        milestone: model.milestone_id.as_deref(),
        health: match model.health {
            ConsistencyHealth::Consistent => "consistent",
            ConsistencyHealth::Inconsistent => "inconsistent",
        },
        semantic_alignment: "not_evaluated",
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
        task_plan_authority: model.task_plan_authority.map(|_| TaskPlanAuthorityData {
            status: "not_current",
            next_action: "reconcile_in_tasks_phase",
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
    json::render(
        &JsonResponse {
            status: "ok",
            code: "SPEC_STATUS_REPORTED",
            data,
        },
        true,
    )
}

fn render_tasks(model: &SpecStatusModel, output: &mut String) {
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

fn render_coverage(model: &SpecStatusModel, output: &mut String) {
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

fn render_diagnostics(model: &SpecStatusModel, output: &mut String) {
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
