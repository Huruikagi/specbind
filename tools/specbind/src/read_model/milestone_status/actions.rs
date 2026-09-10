use super::{
    BTreeMap, BTreeSet, DeliveryStage, ItemFacts, ItemKind, MilestoneAction, MilestoneActionKind,
    MilestoneDiagnostic, MilestoneItemView, MilestoneTaskBlocker, MilestoneTaskProgress, Path,
    ReviewFreshnessStatus, dependencies_ready, design_approved, design_dependencies_ready,
    has_specs, implementation_completion, repository, requirements_approved, spec_predicate,
    spec_status, tasks_approved, tasks_complete, validated,
};

pub(super) fn derive_stage(
    facts: &[ItemFacts],
    review: ReviewFreshnessStatus,
    completion: &BTreeMap<String, bool>,
    reverse: bool,
) -> DeliveryStage {
    let has_specs = has_specs(facts);
    if has_specs && !spec_predicate(facts, requirements_approved) {
        DeliveryStage::Requirements
    } else if has_specs && !spec_predicate(facts, design_approved) {
        DeliveryStage::Design
    } else if !matches!(
        review,
        ReviewFreshnessStatus::Fresh | ReviewFreshnessStatus::NotRequired
    ) {
        DeliveryStage::CrossSpecReview
    } else if reverse {
        DeliveryStage::AdoptionReady
    } else if has_specs && !spec_predicate(facts, tasks_approved) {
        DeliveryStage::Tasks
    } else if !completion.values().all(|complete| *complete) {
        DeliveryStage::Implementation
    } else if has_specs && !spec_predicate(facts, validated) {
        DeliveryStage::Validation
    } else {
        DeliveryStage::ReleasePending
    }
}

#[allow(
    clippy::fn_params_excessive_bools,
    reason = "independent derived predicates remain explicit at this pure scheduling boundary"
)]
pub(super) fn actionable_items(
    facts: &[ItemFacts],
    review: ReviewFreshnessStatus,
    completion: &BTreeMap<String, bool>,
    all_implemented: bool,
    clean: bool,
    release_bound: bool,
    reverse: bool,
) -> Vec<MilestoneAction> {
    let mut actions = Vec::new();
    for item in facts {
        match &item.kind {
            ItemKind::Spec { model, .. }
                if !model.as_deref().is_some_and(requirements_approved) =>
            {
                push_action(&mut actions, item, MilestoneActionKind::Requirements);
            }
            ItemKind::Spec { model, .. }
                if !model.as_deref().is_some_and(design_approved)
                    && design_dependencies_ready(item, facts) =>
            {
                push_action(&mut actions, item, MilestoneActionKind::Design);
            }
            ItemKind::Spec { .. } if reverse => {}
            ItemKind::Spec { model, .. }
                if review == ReviewFreshnessStatus::Fresh
                    && !model.as_deref().is_some_and(tasks_approved) =>
            {
                push_action(&mut actions, item, MilestoneActionKind::Tasks);
            }
            ItemKind::Spec { model, .. }
                if model.as_deref().is_some_and(tasks_approved)
                    && !model.as_deref().is_some_and(tasks_complete)
                    && model.as_deref().is_some_and(|model| {
                        model
                            .task_model
                            .as_ref()
                            .is_some_and(|tasks| !tasks.actionable_ids.is_empty())
                    })
                    && !completion[&item.id]
                    && dependencies_ready(item, completion) =>
            {
                push_action(&mut actions, item, MilestoneActionKind::Implementation);
            }
            ItemKind::Spec { model, .. }
                if all_implemented
                    && completion[&item.id]
                    && dependencies_ready(item, completion)
                    && clean
                    && !model.as_deref().is_some_and(validated) =>
            {
                push_action(&mut actions, item, MilestoneActionKind::Validation);
            }
            ItemKind::Direct { completed }
                if !completed && dependencies_ready(item, completion) =>
            {
                push_action(&mut actions, item, MilestoneActionKind::Implementation);
            }
            _ => {}
        }
    }
    if review != ReviewFreshnessStatus::NotRequired
        && facts.iter().all(|item| match &item.kind {
            ItemKind::Spec { model, .. } => model.as_deref().is_some_and(design_approved),
            ItemKind::Direct { .. } => true,
        })
        && review != ReviewFreshnessStatus::Fresh
    {
        actions.push(MilestoneAction {
            item: "milestone".to_owned(),
            command_operand: None,
            action: MilestoneActionKind::ContractReview,
        });
    }
    if reverse
        && review == ReviewFreshnessStatus::Fresh
        && facts.iter().all(|item| match &item.kind {
            ItemKind::Spec { model, .. } => model.as_deref().is_some_and(design_approved),
            ItemKind::Direct { .. } => true,
        })
    {
        actions.push(MilestoneAction {
            item: "milestone".to_owned(),
            command_operand: None,
            action: MilestoneActionKind::AdoptionFinalize,
        });
    }
    if !reverse
        && matches!(
            review,
            ReviewFreshnessStatus::Fresh | ReviewFreshnessStatus::NotRequired
        )
        && all_implemented
        && facts.iter().all(|item| match &item.kind {
            ItemKind::Spec { model, .. } => model.as_deref().is_some_and(validated),
            ItemKind::Direct { completed } => *completed,
        })
    {
        actions.push(MilestoneAction {
            item: "milestone".to_owned(),
            command_operand: None,
            action: if release_bound {
                MilestoneActionKind::ReleasePreflight
            } else {
                MilestoneActionKind::BindRelease
            },
        });
    }
    actions
}

pub(super) fn worktree_blocks_progress(
    facts: &[ItemFacts],
    review: ReviewFreshnessStatus,
    current_stage: DeliveryStage,
    current_actions: &[MilestoneAction],
    release_bound: bool,
    reverse: bool,
) -> bool {
    let completion = implementation_completion(facts, true);
    let all_implemented = completion.values().all(|complete| *complete);
    let stage = derive_stage(facts, review, &completion, reverse);
    let actions = actionable_items(
        facts,
        review,
        &completion,
        all_implemented,
        true,
        release_bound,
        reverse,
    );
    stage != current_stage || actions != current_actions
}

fn push_action(actions: &mut Vec<MilestoneAction>, item: &ItemFacts, action: MilestoneActionKind) {
    actions.push(MilestoneAction {
        item: item.id.clone(),
        command_operand: Some(item.command_operand.clone()),
        action,
    });
}

pub(super) fn item_views(
    facts: &[ItemFacts],
    completion: &BTreeMap<String, bool>,
) -> Vec<MilestoneItemView> {
    facts
        .iter()
        .map(|item| {
            let status = match &item.kind {
                ItemKind::Spec { model, .. } if model.as_deref().is_some_and(validated) => {
                    "validated".to_owned()
                }
                ItemKind::Spec { model, .. } => model.as_deref().map_or_else(
                    || "unavailable".to_owned(),
                    |model| spec_status::state_name(model.declared_state).to_owned(),
                ),
                ItemKind::Direct { completed: true } => "completed".to_owned(),
                ItemKind::Direct { completed: false } => "pending".to_owned(),
            };
            let waiting_for = item
                .dependencies
                .iter()
                .filter(|dependency| !completion.get(*dependency).copied().unwrap_or(false))
                .cloned()
                .collect();
            let (task_progress, task_blockers) = match &item.kind {
                ItemKind::Spec { model, .. } => model.as_deref().map_or_else(
                    || (None, Vec::new()),
                    |model| {
                        let progress =
                            model
                                .task_model
                                .as_ref()
                                .map(|tasks| MilestoneTaskProgress {
                                    total: tasks.total(),
                                    completed: tasks.completed,
                                    pending: tasks.pending,
                                    blocked: tasks.blocked,
                                    next_tasks: tasks.actionable_ids.clone(),
                                });
                        let blockers = model
                            .blockers
                            .iter()
                            .map(|blocker| MilestoneTaskBlocker {
                                task_id: blocker.task_id.clone(),
                                reason: blocker.reason.clone(),
                            })
                            .collect();
                        (progress, blockers)
                    },
                ),
                ItemKind::Direct { .. } => (None, Vec::new()),
            };
            MilestoneItemView {
                id: item.id.clone(),
                summary: item.summary.clone(),
                status,
                waiting_for,
                task_progress,
                task_blockers,
            }
        })
        .collect()
}

pub(super) fn task_state_checkpointed(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    diagnostics: &mut BTreeSet<MilestoneDiagnostic>,
) -> bool {
    let path = specbind_root.join(format!("specs/{canonical_spec}/tasks.yaml"));
    let Ok(relative) = path.strip_prefix(project_root) else {
        diagnostics.insert(MilestoneDiagnostic {
            code: "MILESTONE_TASK_CHECKPOINT_PATH_INVALID",
            path: Some(format!("specs/{canonical_spec}/tasks.yaml")),
            message: "task state path is outside the project root".to_owned(),
        });
        return false;
    };
    let relative = relative.to_string_lossy().replace('\\', "/");
    match repository::path_status(project_root, &relative) {
        Ok(status) => status.is_empty(),
        Err(error) => {
            diagnostics.insert(MilestoneDiagnostic {
                code: "MILESTONE_TASK_CHECKPOINT_STATUS_FAILED",
                path: Some(relative),
                message: error.to_string(),
            });
            false
        }
    }
}
