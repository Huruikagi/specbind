//! Read model for current milestone status over Roadmap, Specs, review, tasks, and Git.

mod actions;
mod diagnostics;
mod release;

use actions::{
    actionable_items, derive_stage, item_views, task_state_checkpointed, worktree_blocks_progress,
};
use diagnostics::{
    dependencies_ready, dependency_key, design_dependencies_ready,
    diagnose_required_contract_graph, diagnose_unscoped_active_specs, failure, from_discovery,
    git_state, state_rank,
};
use release::{
    derive_release_readiness, has_specs, release_blockers, spec_predicate, spec_state_counts,
};

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use crate::{
    artifacts::{self, DiscoveryIssue},
    contract_graph::{self, GraphIssueSeverity},
    cross_spec_review::{self, ReviewFreshnessStatus},
    freshness::{self, FreshnessStatus},
    repository,
    roadmap::{self, Dependency, DirectStatus, RoadmapDocument},
    schema::spec::v1::WorkflowState,
    spec_status::{self, ConsistencyHealth, SpecStatusModel},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryStage {
    Requirements,
    Design,
    CrossSpecReview,
    AdoptionReady,
    Tasks,
    Implementation,
    Validation,
    ReleasePending,
    ReleaseReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MilestoneHealth {
    Consistent,
    Inconsistent,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MilestoneDiagnostic {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneAction {
    pub item: String,
    pub command_operand: Option<String>,
    pub action: MilestoneActionKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MilestoneActionKind {
    Requirements,
    Design,
    ContractReview,
    SharedContractPlan,
    Tasks,
    Implementation,
    Validation,
    AdoptionFinalize,
    BindRelease,
    ReleasePreflight,
}

impl MilestoneActionKind {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Requirements => "requirements",
            Self::Design => "design",
            Self::ContractReview => "contract_review",
            Self::SharedContractPlan => "shared_contract_plan",
            Self::Tasks => "tasks",
            Self::Implementation => "implementation",
            Self::Validation => "validation",
            Self::AdoptionFinalize => "adoption_finalize",
            Self::BindRelease => "bind_release",
            Self::ReleasePreflight => "release_preflight",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneItemView {
    pub id: String,
    pub summary: String,
    pub status: String,
    pub waiting_for: Vec<String>,
    pub task_progress: Option<MilestoneTaskProgress>,
    pub task_blockers: Vec<MilestoneTaskBlocker>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneTaskProgress {
    pub total: usize,
    pub completed: usize,
    pub pending: usize,
    pub blocked: usize,
    pub next_tasks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneTaskBlocker {
    pub task_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneStatusModel {
    pub milestone_id: String,
    /// The revision this milestone's Contract changes are compared against.
    pub baseline_revision: String,
    pub baseline_version: Option<String>,
    pub target_release: Option<String>,
    pub reverse: bool,
    pub stage: DeliveryStage,
    pub health: MilestoneHealth,
    pub review_status: ReviewFreshnessStatus,
    pub spec_state_counts: BTreeMap<String, usize>,
    pub direct_completed: usize,
    pub direct_total: usize,
    pub current_revision: Option<String>,
    pub items: Vec<MilestoneItemView>,
    pub actionable: Vec<MilestoneAction>,
    pub current_blockers: Vec<String>,
    pub release_blockers: Vec<String>,
    pub diagnostics: Vec<MilestoneDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneStatusFailure {
    pub diagnostics: Vec<MilestoneDiagnostic>,
}

struct ItemFacts {
    id: String,
    command_operand: String,
    summary: String,
    dependencies: Vec<String>,
    kind: ItemKind,
}

enum ItemKind {
    Spec {
        model: Option<Box<SpecStatusModel>>,
        tasks_checkpointed: bool,
    },
    Direct {
        completed: bool,
    },
}

struct GitState {
    revision: Option<String>,
    clean: bool,
    diagnostic: Option<MilestoneDiagnostic>,
}

/// Resolves the one active milestone. Absence is a normal no-change result.
///
/// # Errors
///
/// Returns a fatal failure when the active Roadmap exists but no trustworthy
/// typed scope can be read.
#[allow(
    clippy::too_many_lines,
    reason = "the read model derives one coherent snapshot from all milestone inputs"
)]
pub fn resolve(
    project_root: &Path,
    specbind_root: &Path,
) -> Result<Option<MilestoneStatusModel>, MilestoneStatusFailure> {
    let Some(roadmap) = read_roadmap(specbind_root)? else {
        return Ok(None);
    };
    let git = git_state(project_root);
    let review = cross_spec_review::evaluate_freshness(project_root, specbind_root);
    let mut diagnostics = BTreeSet::new();
    if let Some(diagnostic) = git.diagnostic.clone() {
        diagnostics.insert(diagnostic);
    }
    if review.status != ReviewFreshnessStatus::Missing {
        diagnostics.extend(review.issues.iter().map(|issue| MilestoneDiagnostic {
            code: issue.code,
            path: issue.source.clone(),
            message: issue.message.clone(),
        }));
    }
    let mut facts = spec_facts(project_root, specbind_root, &roadmap, &mut diagnostics);
    facts.extend(direct_facts(&roadmap));
    let reverse = !roadmap.reverse_specs.is_empty();
    diagnose_unscoped_active_specs(specbind_root, &roadmap, &mut diagnostics);
    diagnose_required_contract_graph(specbind_root, &facts, &mut diagnostics);
    if reverse
        && matches!(review.status, ReviewFreshnessStatus::Missing)
        && facts.iter().any(|item| match &item.kind {
            ItemKind::Spec { model, .. } => model
                .as_ref()
                .is_some_and(|model| model.task_model.is_some()),
            ItemKind::Direct { .. } => false,
        })
    {
        diagnostics.insert(MilestoneDiagnostic {
            code: "MILESTONE_REVERSE_TASKS_FORBIDDEN",
            path: None,
            message: "reverse milestone participants cannot carry tasks.yaml".to_owned(),
        });
    }
    let validation_checkout_ready = git.clean
        || git.revision.as_deref().is_some_and(|revision| {
            freshness::assess_pending_completion_mutations(project_root, specbind_root, revision)
                .issues
                .is_empty()
        });
    let implementation_complete = implementation_completion(&facts, git.clean);
    let all_items_implemented = implementation_complete.values().all(|complete| *complete);
    let all_specs_validated = spec_predicate(&facts, validated);
    let stage = derive_stage(&facts, review.status, &implementation_complete, reverse);
    let health = if diagnostics.is_empty() {
        MilestoneHealth::Consistent
    } else {
        MilestoneHealth::Inconsistent
    };
    let mut actionable = actionable_items(
        &facts,
        review.status,
        &implementation_complete,
        all_items_implemented,
        validation_checkout_ready,
        roadmap.target_release.is_some(),
        reverse,
    );
    if !reverse
        && (roadmap.has_shared_changes()
            || cross_spec_review::has_shared_changes(project_root, specbind_root, &roadmap)
                .unwrap_or(false))
        && review.status != ReviewFreshnessStatus::Fresh
    {
        actionable.retain(|action| {
            matches!(
                action.action,
                MilestoneActionKind::Requirements | MilestoneActionKind::Design
            )
        });
        let position = actionable
            .iter()
            .position(|action| action.action == MilestoneActionKind::Design)
            .unwrap_or(actionable.len());
        actionable.insert(
            position,
            MilestoneAction {
                item: "milestone".into(),
                command_operand: None,
                action: MilestoneActionKind::SharedContractPlan,
            },
        );
    }
    let mut current_blockers = Vec::new();
    if facts.iter().any(|item| match &item.kind {
        ItemKind::Spec { model, .. } => model
            .as_deref()
            .is_some_and(|model| !model.blockers.is_empty()),
        ItemKind::Direct { .. } => false,
    }) {
        current_blockers.push("TASKS_BLOCKED".to_owned());
    }
    if !validation_checkout_ready
        && git.diagnostic.is_none()
        && worktree_blocks_progress(
            &facts,
            review.status,
            stage,
            &actionable,
            roadmap.target_release.is_some(),
            reverse,
        )
    {
        current_blockers.push("WORKTREE_NOT_CLEAN".to_owned());
    }
    let items = item_views(&facts, &implementation_complete);
    let release_blockers = if reverse {
        Vec::new()
    } else {
        release_blockers(&facts, &roadmap, review.status, health, all_specs_validated)
    };
    let (stage, release_blockers) = if reverse {
        (stage, release_blockers)
    } else {
        derive_release_readiness(project_root, specbind_root, stage, release_blockers)
    };
    let spec_state_counts = spec_state_counts(&facts);
    let direct_total = roadmap.direct_changes.len();
    let direct_completed = roadmap
        .direct_changes
        .iter()
        .filter(|item| item.status == Some(DirectStatus::Completed))
        .count();

    Ok(Some(MilestoneStatusModel {
        milestone_id: roadmap.milestone_id,
        baseline_revision: roadmap.baseline_revision,
        baseline_version: roadmap.baseline_version,
        target_release: roadmap.target_release,
        reverse,
        stage,
        health,
        review_status: review.status,
        spec_state_counts,
        direct_completed,
        direct_total,
        current_revision: git.revision,
        items,
        actionable,
        current_blockers,
        release_blockers,
        diagnostics: diagnostics.into_iter().collect(),
    }))
}

/// Loads the active Roadmap, distinguishing absence from an unreadable or
/// invalid document. `Ok(None)` means no milestone is active.
///
/// # Errors
///
/// Returns read, path-safety, or parser diagnostics.
pub(crate) fn read_roadmap(
    specbind_root: &Path,
) -> Result<Option<RoadmapDocument>, MilestoneStatusFailure> {
    Ok(read_roadmap_source(specbind_root)?.map(|(_, document)| document))
}

/// Loads the active Roadmap and retains its original source, which a caller
/// needs to recover the authored Markdown body the parser does not carry.
///
/// # Errors
///
/// Returns read, path-safety, or parser diagnostics.
pub(crate) fn read_roadmap_source(
    specbind_root: &Path,
) -> Result<Option<(String, RoadmapDocument)>, MilestoneStatusFailure> {
    let path = specbind_root.join("steering/roadmap.md");
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(failure("MILESTONE_ROADMAP_READ_FAILED", error.to_string())),
    };
    if !crate::guarded_fs::is_regular_file(&metadata) {
        return Err(failure(
            "MILESTONE_ROADMAP_NOT_REGULAR",
            "steering/roadmap.md must be a regular non-symlink file",
        ));
    }
    let content = fs::read_to_string(path)
        .map_err(|error| failure("MILESTONE_ROADMAP_READ_FAILED", error.to_string()))?;
    roadmap::parse(&content)
        .map(|document| Some((content.clone(), document)))
        .map_err(|error| MilestoneStatusFailure {
            diagnostics: error
                .issues
                .into_iter()
                .map(|issue| MilestoneDiagnostic {
                    code: issue.code,
                    path: Some("steering/roadmap.md".to_owned()),
                    message: issue.message,
                })
                .collect(),
        })
}

fn spec_facts(
    project_root: &Path,
    specbind_root: &Path,
    roadmap: &RoadmapDocument,
    diagnostics: &mut BTreeSet<MilestoneDiagnostic>,
) -> Vec<ItemFacts> {
    roadmap
        .new_specs
        .iter()
        .chain(&roadmap.spec_updates)
        .chain(&roadmap.reverse_specs)
        .map(|item| {
            let id = format!("spec:{}", item.spec);
            let model = match spec_status::resolve(project_root, specbind_root, &item.spec) {
                Ok(model) => {
                    if model.milestone_id.as_deref() != Some(&roadmap.milestone_id) {
                        diagnostics.insert(MilestoneDiagnostic {
                            code: "MILESTONE_SPEC_IDENTITY_MISMATCH",
                            path: Some(format!("specs/{}/spec.yaml", item.spec)),
                            message: format!(
                                "{} active milestone does not match the Roadmap",
                                item.spec
                            ),
                        });
                    }
                    if model.health == ConsistencyHealth::Inconsistent {
                        diagnostics.insert(MilestoneDiagnostic {
                            code: "MILESTONE_SPEC_INCONSISTENT",
                            path: Some(format!("specs/{}/spec.yaml", item.spec)),
                            message: format!("{} has inconsistent status diagnostics", item.spec),
                        });
                    }
                    Some(model)
                }
                Err(error) => {
                    diagnostics.extend(error.issues.iter().map(from_discovery));
                    None
                }
            };
            let tasks_checkpointed = model.as_ref().is_some_and(|model| {
                model.task_model.is_some()
                    && task_state_checkpointed(project_root, specbind_root, &item.spec, diagnostics)
            });
            ItemFacts {
                id,
                command_operand: item.spec.clone(),
                summary: item.summary.clone(),
                dependencies: item.depends_on.iter().map(dependency_key).collect(),
                kind: ItemKind::Spec {
                    model: model.map(Box::new),
                    tasks_checkpointed,
                },
            }
        })
        .collect()
}

fn direct_facts(roadmap: &RoadmapDocument) -> Vec<ItemFacts> {
    roadmap
        .direct_changes
        .iter()
        .map(|item| ItemFacts {
            id: format!("direct:{}", item.id),
            command_operand: item.id.clone(),
            summary: item.summary.clone(),
            dependencies: item.depends_on.iter().map(dependency_key).collect(),
            kind: ItemKind::Direct {
                completed: item.status == Some(DirectStatus::Completed),
            },
        })
        .collect()
}

fn requirements_approved(model: &SpecStatusModel) -> bool {
    state_rank(model.declared_state) >= state_rank(Some(WorkflowState::Design))
        && model.freshness.requirements.status == FreshnessStatus::Fresh
}

fn design_approved(model: &SpecStatusModel) -> bool {
    state_rank(model.declared_state) >= state_rank(Some(WorkflowState::AdoptionReady))
        && model.freshness.requirements.status == FreshnessStatus::Fresh
        && model.freshness.design.status == FreshnessStatus::Fresh
}

fn tasks_approved(model: &SpecStatusModel) -> bool {
    state_rank(model.declared_state) >= state_rank(Some(WorkflowState::Implementation))
        && model.freshness.requirements.status == FreshnessStatus::Fresh
        && model.freshness.design.status == FreshnessStatus::Fresh
        && model.freshness.tasks.status == FreshnessStatus::Fresh
}

fn tasks_complete(model: &SpecStatusModel) -> bool {
    model.task_model.as_ref().is_some_and(|tasks| {
        tasks.completed == tasks.total() && tasks.pending == 0 && tasks.blocked == 0
    })
}

fn validated(model: &SpecStatusModel) -> bool {
    model.declared_state == Some(WorkflowState::ReleaseReady)
        && model.freshness.completion.status == FreshnessStatus::Fresh
}

fn implementation_completion(
    facts: &[ItemFacts],
    assume_clean_checkout: bool,
) -> BTreeMap<String, bool> {
    facts
        .iter()
        .map(|item| {
            let complete = match &item.kind {
                ItemKind::Spec {
                    model,
                    tasks_checkpointed,
                } => {
                    model.as_deref().is_some_and(tasks_complete)
                        && (assume_clean_checkout || *tasks_checkpointed)
                }
                ItemKind::Direct { completed } => *completed,
            };
            (item.id.clone(), complete)
        })
        .collect()
}

#[must_use]
pub fn stage_name(stage: DeliveryStage) -> &'static str {
    match stage {
        DeliveryStage::Requirements => "requirements",
        DeliveryStage::Design => "design",
        DeliveryStage::CrossSpecReview => "contract_review",
        DeliveryStage::AdoptionReady => "adoption_ready",
        DeliveryStage::Tasks => "tasks",
        DeliveryStage::Implementation => "implementation",
        DeliveryStage::Validation => "validation",
        DeliveryStage::ReleasePending => "release_pending",
        DeliveryStage::ReleaseReady => "release_ready",
    }
}

#[must_use]
pub fn review_name(status: ReviewFreshnessStatus) -> &'static str {
    match status {
        ReviewFreshnessStatus::NotRequired => "not_applicable",
        ReviewFreshnessStatus::Missing => "absent",
        ReviewFreshnessStatus::Fresh => "fresh",
        ReviewFreshnessStatus::Stale => "stale",
        ReviewFreshnessStatus::Invalid => "invalid",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        freshness::{ArtifactFreshnessReport, GateFreshness},
        spec_status::WorkflowAction,
        task_read_model::TaskReadModel,
    };

    fn fresh_gate() -> GateFreshness {
        GateFreshness {
            status: FreshnessStatus::Fresh,
            issues: Vec::new(),
        }
    }

    fn implementation_model(
        completed: usize,
        pending: usize,
        blocked: usize,
        actionable_ids: Vec<String>,
    ) -> Box<SpecStatusModel> {
        Box::new(SpecStatusModel {
            declared_state: Some(WorkflowState::Implementation),
            milestone_id: Some("milestone".to_owned()),
            health: ConsistencyHealth::Consistent,
            freshness: ArtifactFreshnessReport {
                requirements: fresh_gate(),
                design: fresh_gate(),
                tasks: fresh_gate(),
                completion: GateFreshness {
                    status: FreshnessStatus::NotReached,
                    issues: Vec::new(),
                },
            },
            contract_review: Some(ReviewFreshnessStatus::Fresh),
            next_action: WorkflowAction::Implementation,
            expected_requirements_work: false,
            expected_design_work: None,
            task_plan_authority: None,
            delegated_gates: Some(Vec::new()),
            task_model: Some(TaskReadModel {
                items: Vec::new(),
                completed,
                pending,
                blocked,
                actionable_ids,
            }),
            blockers: Vec::new(),
            coverage: None,
            diagnostics: Vec::new(),
        })
    }

    #[test]
    fn checkpointed_predecessor_stays_complete_while_a_later_item_is_dirty() {
        let facts = vec![
            ItemFacts {
                id: "spec:upstream".to_owned(),
                command_operand: "upstream".to_owned(),
                summary: "Upstream".to_owned(),
                dependencies: Vec::new(),
                kind: ItemKind::Spec {
                    model: Some(implementation_model(1, 0, 0, Vec::new())),
                    tasks_checkpointed: true,
                },
            },
            ItemFacts {
                id: "spec:downstream".to_owned(),
                command_operand: "downstream".to_owned(),
                summary: "Downstream".to_owned(),
                dependencies: vec!["spec:upstream".to_owned()],
                kind: ItemKind::Spec {
                    model: Some(implementation_model(0, 0, 1, Vec::new())),
                    tasks_checkpointed: false,
                },
            },
        ];

        let completion = implementation_completion(&facts, false);

        assert!(completion["spec:upstream"]);
        assert!(!completion["spec:downstream"]);
        assert!(dependencies_ready(&facts[1], &completion));
        assert!(
            actionable_items(
                &facts,
                ReviewFreshnessStatus::Fresh,
                &completion,
                false,
                false,
                false,
                false,
            )
            .is_empty(),
            "a blocked Spec with no actionable Task must not be redispatched"
        );
    }

    #[test]
    fn completed_spec_waits_for_milestone_convergence_before_validation() {
        let facts = vec![
            ItemFacts {
                id: "spec:completed".to_owned(),
                command_operand: "completed".to_owned(),
                summary: "Completed".to_owned(),
                dependencies: Vec::new(),
                kind: ItemKind::Spec {
                    model: Some(implementation_model(1, 0, 0, Vec::new())),
                    tasks_checkpointed: true,
                },
            },
            ItemFacts {
                id: "spec:blocked".to_owned(),
                command_operand: "blocked".to_owned(),
                summary: "Blocked".to_owned(),
                dependencies: Vec::new(),
                kind: ItemKind::Spec {
                    model: Some(implementation_model(0, 0, 1, Vec::new())),
                    tasks_checkpointed: false,
                },
            },
        ];
        let completion = implementation_completion(&facts, true);

        let actions = actionable_items(
            &facts,
            ReviewFreshnessStatus::Fresh,
            &completion,
            false,
            true,
            false,
            false,
        );

        assert!(
            actions.is_empty(),
            "validation must wait until every milestone item is implementation-complete"
        );
    }
}
