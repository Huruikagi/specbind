use super::super::super::*;
use super::json::{self, JsonResponse};
use serde::Serialize;

#[derive(Serialize)]
struct StatusDiagnosticData<'a> {
    code: &'static str,
    path: Option<&'a str>,
    message: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TaskBlockerData<'a> {
    task_id: &'a str,
    reason: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MilestoneStatusData<'a> {
    milestone_id: &'a str,
    milestone_kind: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline_version: Option<&'a str>,
    target_release: Option<&'a str>,
    stage: &'static str,
    health: &'static str,
    semantic_alignment: &'static str,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    task_progress: Option<MilestoneTaskProgressData<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    task_blockers: Vec<TaskBlockerData<'a>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MilestoneTaskProgressData<'a> {
    total: usize,
    completed: usize,
    pending: usize,
    blocked: usize,
    next_tasks: &'a [String],
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MilestoneActionData<'a> {
    item: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    command_operand: Option<&'a str>,
    action: &'static str,
    handler: MilestoneHandlerData,
}

#[derive(Serialize)]
struct MilestoneHandlerData {
    kind: &'static str,
    target: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<&'static str>,
}

#[must_use]
pub fn milestone_status(start: &Path, json_output: bool) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) if json_output => {
            return json::render_failure(error.code, error.message, vec![]);
        }
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match milestone_status::resolve(&paths.project_root, &paths.specbind_root) {
        Ok(Some(model)) if json_output => render_json(&model),
        Ok(Some(model)) => render_text(&model),
        Ok(None) if json_output => json::render(
            &JsonResponse {
                status: "no_change",
                code: "NO_ACTIVE_MILESTONE",
                data: Option::<()>::None,
            },
            true,
        ),
        Ok(None) => CommandOutput::no_change("NO_ACTIVE_MILESTONE", "No active milestone exists."),
        Err(error) if json_output => json::render_failure(
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

fn render_text(model: &MilestoneStatusModel) -> CommandOutput {
    let mut output = format!(
        "OK MILESTONE_STATUS_REPORTED: Reported the active milestone.\n  Milestone: {}\n",
        escape(&model.milestone_id)
    );
    push_field(&mut output, "Milestone kind", milestone_kind(model));
    push_field(
        &mut output,
        "Baseline version",
        model.baseline_version.as_deref().unwrap_or("none"),
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
        "State health",
        match model.health {
            MilestoneHealth::Consistent => "consistent",
            MilestoneHealth::Inconsistent => "inconsistent",
        },
    );
    push_field(&mut output, "Semantic alignment", "not evaluated");
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
    render_items(model, &mut output);
    render_actions(model, &mut output);
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
    if model.stage == milestone_status::DeliveryStage::AdoptionReady {
        push_field(
            &mut output,
            "Release readiness",
            "not applicable to reverse adoption",
        );
    } else if release_readiness_evaluated(model.stage) {
        push_inline_list(&mut output, "Release blockers", &model.release_blockers);
    } else {
        push_field(
            &mut output,
            "Release readiness",
            "not evaluated until validation",
        );
    }
    render_diagnostics(model, &mut output);
    CommandOutput::success(output.into_bytes())
}

fn render_json(model: &MilestoneStatusModel) -> CommandOutput {
    let release_readiness_evaluated = release_readiness_evaluated(model.stage);
    let data = MilestoneStatusData {
        milestone_id: &model.milestone_id,
        milestone_kind: milestone_kind(model),
        baseline_version: model.baseline_version.as_deref(),
        target_release: model.target_release.as_deref(),
        stage: milestone_status::stage_name(model.stage),
        health: match model.health {
            MilestoneHealth::Consistent => "consistent",
            MilestoneHealth::Inconsistent => "inconsistent",
        },
        semantic_alignment: "not_evaluated",
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
                task_progress: item.task_progress.as_ref().map(|progress| {
                    MilestoneTaskProgressData {
                        total: progress.total,
                        completed: progress.completed,
                        pending: progress.pending,
                        blocked: progress.blocked,
                        next_tasks: &progress.next_tasks,
                    }
                }),
                task_blockers: item
                    .task_blockers
                    .iter()
                    .map(|blocker| TaskBlockerData {
                        task_id: &blocker.task_id,
                        reason: &blocker.reason,
                    })
                    .collect(),
            })
            .collect(),
        actionable: model
            .actionable
            .iter()
            .map(|action| MilestoneActionData {
                item: &action.item,
                command_operand: action.command_operand.as_deref(),
                action: action.action.name(),
                handler: milestone_handler(model.reverse, action.action),
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
    json::render(
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

fn render_items(model: &MilestoneStatusModel, output: &mut String) {
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
        if let Some(progress) = &item.task_progress {
            writeln!(
                output,
                "      Tasks: {}/{} completed, {} pending, {} blocked",
                progress.completed, progress.total, progress.pending, progress.blocked
            )
            .expect("writing to a String cannot fail");
            let next_tasks = if progress.next_tasks.is_empty() {
                "none".to_owned()
            } else {
                progress.next_tasks.join(", ")
            };
            writeln!(output, "      Next tasks: {}", escape(&next_tasks))
                .expect("writing to a String cannot fail");
        }
        for blocker in &item.task_blockers {
            writeln!(
                output,
                "      Blocked task {}: {}",
                escape(&blocker.task_id),
                escape(&blocker.reason)
            )
            .expect("writing to a String cannot fail");
        }
    }
}

fn render_actions(model: &MilestoneStatusModel, output: &mut String) {
    if model.actionable.is_empty() {
        push_field(output, "Actionable", "none");
        return;
    }
    output.push_str("  Actionable:\n");
    for action in &model.actionable {
        let handler = milestone_handler(model.reverse, action.action);
        let mode = handler
            .mode
            .map_or_else(String::new, |mode| format!(" mode={mode}"));
        if let Some(command_operand) = &action.command_operand {
            writeln!(
                output,
                "    - {} action={} command_operand={} handler={}:{}{}",
                escape(&action.item),
                action.action.name(),
                escape(command_operand),
                handler.kind,
                handler.target,
                mode,
            )
            .expect("writing to a String cannot fail");
        } else {
            writeln!(
                output,
                "    - {} action={} handler={}:{}{}",
                escape(&action.item),
                action.action.name(),
                handler.kind,
                handler.target,
                mode,
            )
            .expect("writing to a String cannot fail");
        }
    }
}

fn milestone_kind(model: &MilestoneStatusModel) -> &'static str {
    if model.reverse { "reverse" } else { "delivery" }
}

fn milestone_handler(
    reverse: bool,
    action: milestone_status::MilestoneActionKind,
) -> MilestoneHandlerData {
    use milestone_status::MilestoneActionKind as Action;

    if reverse {
        return MilestoneHandlerData {
            kind: "skill",
            target: "sb-adopt",
            mode: Some("reverse_resume"),
        };
    }
    match action {
        Action::Requirements | Action::Design | Action::Tasks => MilestoneHandlerData {
            kind: "skill",
            target: "sb-plan",
            mode: Some("all_spec"),
        },
        Action::SharedContractPlan => MilestoneHandlerData {
            kind: "skill",
            target: "sb-plan",
            mode: Some("shared"),
        },
        Action::ContractReview => MilestoneHandlerData {
            kind: "skill",
            target: "sb-contract-review",
            mode: None,
        },
        Action::Implementation => MilestoneHandlerData {
            kind: "skill",
            target: "sb-implement",
            mode: Some("item"),
        },
        Action::Validation => MilestoneHandlerData {
            kind: "skill",
            target: "sb-validate-implementation",
            mode: Some("item"),
        },
        Action::BindRelease => MilestoneHandlerData {
            kind: "guarded_cli",
            target: "specbind milestone bind-release",
            mode: None,
        },
        Action::ReleasePreflight => MilestoneHandlerData {
            kind: "boundary",
            target: "sb-release",
            mode: None,
        },
        Action::AdoptionFinalize => unreachable!("reverse-only action handled above"),
    }
}

fn render_diagnostics(model: &MilestoneStatusModel, output: &mut String) {
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

#[cfg(test)]
mod handler_tests {
    use super::*;

    #[test]
    fn every_projected_skill_handler_names_an_embedded_skill() {
        use milestone_status::MilestoneActionKind as Action;

        let delivery_actions = [
            Action::Requirements,
            Action::Design,
            Action::ContractReview,
            Action::Tasks,
            Action::Implementation,
            Action::Validation,
            Action::BindRelease,
            Action::ReleasePreflight,
        ];
        let handlers = delivery_actions
            .into_iter()
            .map(|action| milestone_handler(false, action))
            .chain([milestone_handler(true, Action::AdoptionFinalize)]);

        for handler in handlers.filter(|handler| handler.kind == "skill") {
            assert!(
                crate::skill::find(handler.target).is_some(),
                "handler target {} must be an embedded Skill",
                handler.target
            );
        }
    }
}
